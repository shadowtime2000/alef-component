// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use crate::ast_walker::ast_walker;
use crate::error::{DiagnosticBuffer, ErrorBuffer};
use crate::resolve::Resolver;

use serde::Deserialize;
use std::{cell::RefCell, path::Path, rc::Rc};
use swc_common::{
  chain,
  comments::SingleThreadedComments,
  errors::{Handler, HandlerFlags},
  FileName, Globals, SourceMap,
};
use swc_ecmascript::{
  ast::{Module, Program},
  codegen::{text_writer::JsWriter, Node},
  parser::lexer::Lexer,
  parser::{JscTarget, StringInput, Syntax, TsConfig},
  transforms::{fixer, helpers, react, typescript},
  visit::{Fold, FoldWith},
};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EmitOptions {
  #[serde(default = "default_helper_module")]
  pub helper_module: String,

  #[serde(default = "default_target")]
  pub target: JscTarget,

  #[serde(default)]
  pub source_map: bool,

  #[serde(default)]
  pub is_dev: bool,
}

fn default_helper_module() -> String {
  "@alephjs/helper".into()
}

fn default_target() -> JscTarget {
  JscTarget::Es2020
}

#[derive(Clone)]
pub struct AlefComponentModule {
  pub specifier: String,
  pub module: Module,
  pub source_map: Rc<SourceMap>,
  pub comments: SingleThreadedComments,
}

impl AlefComponentModule {
  /// Parse Alef Component to AST.
  pub fn parse(specifier: &str, source: &str) -> Result<Self, anyhow::Error> {
    let source_map = SourceMap::default();
    let source_file = source_map.new_source_file(
      FileName::Real(Path::new(specifier).to_path_buf()),
      source.into(),
    );
    let sm = &source_map;
    let error_buffer = ErrorBuffer::new();
    let syntax = Syntax::Typescript(TsConfig {
      tsx: true,
      dynamic_import: true,
      decorators: false,
      ..TsConfig::default()
    });
    let input = StringInput::from(&*source_file);
    let comments = SingleThreadedComments::default();
    let lexer = Lexer::new(syntax, JscTarget::Es2020, input, Some(&comments));
    let mut parser = swc_ecmascript::parser::Parser::new_from(lexer);
    let handler = Handler::with_emitter_and_flags(
      Box::new(error_buffer.clone()),
      HandlerFlags {
        can_emit_warnings: true,
        dont_buffer_diagnostics: true,
        ..HandlerFlags::default()
      },
    );
    let module = parser
      .parse_module()
      .map_err(move |err| {
        let mut diagnostic = err.into_diagnostic(&handler);
        diagnostic.emit();
        DiagnosticBuffer::from_error_buffer(error_buffer, |span| sm.lookup_char_pos(span.lo))
      })
      .unwrap();

    Ok(AlefComponentModule {
      specifier: specifier.into(),
      module,
      source_map: Rc::new(source_map),
      comments,
    })
  }

  /// Transform Alef Component into Javascript.
  pub fn transpile(
    self,
    resolver: Rc<RefCell<Resolver>>,
    options: &EmitOptions,
  ) -> Result<(String, Option<String>), anyhow::Error> {
    let mut passes = chain!(
      ast_walker(resolver.clone(), self.source_map.clone(), options.is_dev),
      react::jsx(
        self.source_map.clone(),
        Some(&self.comments),
        react::Options {
          use_builtins: true,
          ..Default::default()
        },
      ),
      typescript::strip(),
      fixer(Some(&self.comments)),
    );

    self.apply_transform(&mut passes, options.source_map)
  }

  /// Apply transform with given fold.
  pub fn apply_transform<T: Fold>(
    &self,
    mut tr: T,
    source_map: bool,
  ) -> Result<(String, Option<String>), anyhow::Error> {
    let program = Program::Module(self.module.clone());
    let program = swc_common::GLOBALS.set(&Globals::new(), || {
      helpers::HELPERS.set(&helpers::Helpers::new(false), || program.fold_with(&mut tr))
    });
    let mut buf = Vec::new();
    let mut src_map_buf = Vec::new();
    let src_map = if source_map {
      Some(&mut src_map_buf)
    } else {
      None
    };
    {
      let writer = Box::new(JsWriter::new(
        self.source_map.clone(),
        "\n",
        &mut buf,
        src_map,
      ));
      let mut emitter = swc_ecmascript::codegen::Emitter {
        cfg: swc_ecmascript::codegen::Config {
          minify: false, // todo: use swc minify in the future, currently use terser
        },
        comments: Some(&self.comments),
        cm: self.source_map.clone(),
        wr: writer,
      };
      program.emit_with(&mut emitter).unwrap();
    }
    let src = String::from_utf8(buf).unwrap();
    if source_map {
      let mut buf = Vec::new();
      self
        .source_map
        .build_source_map_from(&mut src_map_buf, None)
        .to_writer(&mut buf)
        .unwrap();
      Ok((src, Some(String::from_utf8(buf).unwrap())))
    } else {
      Ok((src, None))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::resolve::Resolver;

  fn t(specifer: &str, source: &str) -> (String, Rc<RefCell<Resolver>>) {
    let module = AlefComponentModule::parse(specifer, source).expect("could not parse module");
    let resolver = Rc::new(RefCell::new(Resolver::new()));
    let (code, _) = module
      .transpile(resolver.clone(), &EmitOptions::default())
      .expect("could not transpile module");
    (code, resolver)
  }

  #[test]
  fn test_transpile_ts() {
    let source = r#"
      let name: string = 'world'

      $t: <p>hello {name}!</p>    
    "#;
    let (code, _) = t("App.alef", source);
    println!("{}", code);
    assert!(code.contains("let name = 'world';"));
  }
}
