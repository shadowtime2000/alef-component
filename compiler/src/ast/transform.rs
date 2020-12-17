// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::{statement::*, AST};
use crate::resolve::{format_component_name, Resolver};
use indexmap::IndexSet;
use std::{cell::RefCell, path::Path, rc::Rc};
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_utils::quote_ident;
use swc_ecma_visit::{noop_fold_type, Fold};

/// AST Transform for Alef Component.
pub struct ASTransform {
  pub resolver: Rc<RefCell<Resolver>>,
}

impl ASTransform {
  pub fn transform(&self, ast: &AST) -> Vec<Stmt> {
    let mut scope_idents: IndexSet<&Str> = IndexSet::new();
    let mut stmts: Vec<Stmt> = vec![];
    stmts
  }
}

impl Fold for ASTransform {
  noop_fold_type!();

  fn fold_module_items(&mut self, _: Vec<ModuleItem>) -> Vec<ModuleItem> {
    let resolver = self.resolver.borrow_mut();
    let mut output: Vec<ModuleItem> = vec![];

    // import dom helper module
    if resolver.dom_helper_module.starts_with("window.") {
      let mut props: Vec<ObjectPatProp> = vec![];
      for (name, rename) in resolver.dep_helpers.clone() {
        match rename {
          Some(rename) => props.push(ObjectPatProp::KeyValue(KeyValuePatProp {
            key: PropName::Ident(quote_ident!(name)),
            value: Box::new(Pat::Ident(quote_ident!(rename))),
          })),
          _ => props.push(ObjectPatProp::Assign(AssignPatProp {
            span: DUMMY_SP,
            key: quote_ident!(name),
            value: None,
          })),
        }
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
          init: Some(Box::new(Expr::MetaProp(MetaPropExpr {
            meta: quote_ident!("window"),
            prop: quote_ident!(resolver.dom_helper_module.trim_start_matches("window.")),
          }))),
          definite: false,
        }],
      }))));
    } else {
      let mut specifiers: Vec<ImportSpecifier> = vec![];
      for (name, rename) in resolver.dep_helpers.clone() {
        specifiers.push(ImportSpecifier::Named(ImportNamedSpecifier {
          span: DUMMY_SP,
          local: quote_ident!(name),
          imported: match rename {
            Some(rename) => Some(quote_ident!(rename)),
            _ => None,
          },
        }))
      }
      output.push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers,
        src: Str {
          span: DUMMY_SP,
          value: resolver.dom_helper_module.as_str().into(),
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
                  pat: Pat::Ident(quote_ident!("props")),
                })],
                body: Some(BlockStmt {
                  span: DUMMY_SP,
                  stmts: match &resolver.ast {
                    Some(ast) => [
                      vec![Stmt::Expr(ExprStmt {
                        span: DUMMY_SP,
                        expr: Box::new(Expr::Call(CallExpr {
                          span: DUMMY_SP,
                          callee: ExprOrSuper::Super(Super { span: DUMMY_SP }),
                          args: vec![ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::Ident(quote_ident!("props"))),
                          }],
                          type_args: None,
                        })),
                      })],
                      self.transform(ast),
                    ]
                    .concat(),
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

fn get_idents_from_pat(pat: &Pat) -> Vec<Ident> {
  let mut idents: Vec<Ident> = vec![];

  match pat {
    Pat::Ident(id) => {
      idents.push(id.clone());
    }
    Pat::Array(ArrayPat { elems, .. }) => {
      for el in elems {
        match el {
          Some(el) => {
            for id in get_idents_from_pat(el) {
              idents.push(id);
            }
          }
          _ => {}
        }
      }
    }
    Pat::Object(ObjectPat { props, .. }) => {
      for prop in props {
        match prop {
          ObjectPatProp::Assign(AssignPatProp { key, .. }) => idents.push(key.clone()),
          ObjectPatProp::KeyValue(KeyValuePatProp { value, .. }) => {
            for id in get_idents_from_pat(value.as_ref()) {
              idents.push(id)
            }
          }
          ObjectPatProp::Rest(RestPat { arg, .. }) => {
            for id in get_idents_from_pat(arg.as_ref()) {
              idents.push(id)
            }
          }
        }
      }
    }
    _ => {}
  };

  idents
}
