// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::ast::ast_trasnform;
use super::resolve::{format_component_name, Resolver};
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

    // import helper module
    if resolver.helper_module.starts_with("window.") {
      let mut props: Vec<ObjectPatProp> = vec![];
      for name in resolver.dep_helpers.clone() {
        props.push(ObjectPatProp::Assign(AssignPatProp {
          span: DUMMY_SP,
          key: quote_ident!(name),
          value: None,
        }))
      }
      output.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
          span: DUMMY_SP,
          name: Pat::Object(ObjectPat {
            span: DUMMY_SP,
            props,
            optional: false,
            type_ann: None,
          }),
          init: Some(Box::new(Expr::Ident(quote_ident!(resolver
            .helper_module
            .trim_start_matches("window."))))),
          definite: false,
        }],
      }))));
    } else {
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
                  stmts: match &resolver.ast {
                    Some(ast) => ast_trasnform(ast),
                    _ => vec![],
                  },
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
