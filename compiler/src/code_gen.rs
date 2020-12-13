// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use crate::resolve::Resolver;
use std::path::Path;
use std::{cell::RefCell, rc::Rc};
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_utils::quote_ident;
use swc_ecma_visit::{noop_fold_type, Fold};

pub fn code_gen(resolver: Rc<RefCell<Resolver>>) -> impl Fold {
  CodeGen {
    resolver: resolver.clone(),
  }
}

/// Code generator for Alef Component.
struct CodeGen {
  resolver: Rc<RefCell<Resolver>>,
}

impl Fold for CodeGen {
  noop_fold_type!();

  fn fold_module_items(&mut self, _: Vec<ModuleItem>) -> Vec<ModuleItem> {
    let resolver = self.resolver.borrow_mut();
    let mut output: Vec<ModuleItem> = vec![];
    let mut stmts: Vec<Stmt> = vec![]; // todo: transform AST to stmts

    // import helper module
    {
      let mut specifiers: Vec<ImportSpecifier> = vec![];
      for name in resolver.dep_helpers.clone() {
        specifiers.push(ImportSpecifier::Named(ImportNamedSpecifier {
          span: DUMMY_SP,
          local: quote_ident!(name),
          imported: None,
        }))
      }
      output.push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers,
        src: Str {
          span: DUMMY_SP,
          value: resolver.helper_module.as_str().into(),
          has_escape: false,
        },
        type_only: false,
        asserts: None,
      })));
    }

    // export component
    {
      let path = Path::new(resolver.specifier.as_str());
      let file_name = path.file_name().as_ref().unwrap().to_str().unwrap();
      let name = format_component_name(file_name);
      output.push(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
        ExportDefaultExpr {
          span: DUMMY_SP,
          expr: Box::new(Expr::Class(ClassExpr {
            ident: Some(quote_ident!(name)),
            class: Class {
              span: DUMMY_SP,
              decorators: vec![],
              body: vec![ClassMember::Constructor(Constructor {
                span: DUMMY_SP,
                key: PropName::Ident(quote_ident!("constructor")),
                params: vec![ParamOrTsParamProp::Param(Param {
                  span: DUMMY_SP,
                  decorators: vec![],
                  pat: Pat::Ident(quote_ident!("prop")),
                })],
                body: Some(BlockStmt {
                  span: DUMMY_SP,
                  stmts,
                }),
                accessibility: None,
                is_optional: false,
              })],
              super_class: Some(Box::new(Expr::Ident(quote_ident!("Component")))),
              is_abstract: false,
              type_params: None,
              super_type_params: None,
              implements: vec![],
            },
          })),
        },
      )));
    }

    output
  }
}

fn format_component_name(s: &str) -> String {
  let mut should_uppercase = true;
  let mut char_vec: Vec<char> = vec![];
  for c in s.trim_end_matches(".alef").chars() {
    if c >= 'a' && c <= 'z' {
      if should_uppercase {
        should_uppercase = false;
        char_vec.push(c.to_ascii_uppercase());
      } else {
        char_vec.push(c);
      }
    } else if c >= 'A' && c <= 'Z' {
      should_uppercase = false;
      char_vec.push(c);
    } else if (c >= '0' && c <= '9') && char_vec.len() > 0 {
      should_uppercase = false;
      char_vec.push(c);
    } else {
      should_uppercase = true
    }
  }
  if char_vec.len() == 0 {
    return "App".into();
  }
  char_vec.into_iter().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_format_component_name() {
    assert_eq!(format_component_name("app.alef"), "App");
    assert_eq!(format_component_name("hello-world.alef"), "HelloWorld");
    assert_eq!(format_component_name("hello_world.alef"), "HelloWorld");
    assert_eq!(format_component_name("hello.world.alef"), "HelloWorld");
    assert_eq!(format_component_name("hello world.alef"), "HelloWorld");
    assert_eq!(format_component_name("HELLO world.alef"), "HELLOWorld");
    assert_eq!(format_component_name("h798.alef"), "H798");
    assert_eq!(format_component_name("798hello world.alef"), "HelloWorld");
    assert_eq!(format_component_name("798.alef"), "App");
    assert_eq!(format_component_name("Hello 世界!.alef"), "Hello");
  }
}
