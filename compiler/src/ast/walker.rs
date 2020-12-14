// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::statement::*;
use super::AST;
use super::{css::CSS, jsx::JSX};
use crate::resolve::Resolver;
use indexmap::IndexSet;
use std::{cell::RefCell, rc::Rc};
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_fold_type, Fold};

/// AST walker for Alef Component.
pub struct ASTWalker {
  pub resolver: Rc<RefCell<Resolver>>,
}

/// AST walker walks the component to an AST then stores it in resolver,
/// and returns a empty module.
impl Fold for ASTWalker {
  noop_fold_type!();

  fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    let mut resolver = self.resolver.borrow_mut();
    let mut stmts: Vec<Statement> = vec![];
    let mut states: IndexSet<String> = IndexSet::new();

    for item in module_items {
      match item {
        ModuleItem::ModuleDecl(decl) => {}
        ModuleItem::Stmt(stmt) => match stmt {
          Stmt::Decl(Decl::Var(VarDecl { kind, decls, .. })) => match kind {
            VarDeclKind::Const => for decl in decls {},
            _ => {
              for decl in decls {
                let mut is_array = false;
                let mut is_ref = false;
                let is_async = match decl.init {
                  Some(ref expr) => match expr.as_ref() {
                    Expr::Await(_) => true,
                    _ => false,
                  },
                  _ => false,
                };
                match decl.name {
                  Pat::Ident(Ident { ref type_ann, .. })
                  | Pat::Array(ArrayPat { ref type_ann, .. })
                  | Pat::Object(ObjectPat { ref type_ann, .. }) => match type_ann {
                    Some(TsTypeAnn { type_ann, .. }) => match type_ann.as_ref() {
                      TsType::TsArrayType(_) => is_array = true,
                      TsType::TsTypeRef(TsTypeRef {
                        type_name: TsEntityName::Ident(Ident { sym, .. }),
                        ..
                      }) => is_ref = sym.as_ref().eq("Ref"),
                      _ => {}
                    },
                    _ => {}
                  },
                  _ => {}
                };
                if !is_ref {
                  for id in get_idents_from_pat(&decl.name) {
                    states.insert(id.sym.as_ref().into());
                  }
                }
                stmts.push(Statement::Var(VarStatement {
                  name: decl.name,
                  expr: decl.init,
                  is_array,
                  is_ref,
                  is_async,
                }))
              }
            }
          },
          Stmt::Labeled(labeled) => match labeled.label.as_ref() {
            "$" => stmts.push(Statement::SideEffect(SideEffectStatement {
              name: None,
              stmt: labeled.body,
            })),
            "$t" => match labeled.body.as_ref() {
              Stmt::Expr(ExprStmt { expr, .. }) => match expr.as_ref() {
                Expr::JSXElement(el) => stmts.push(Statement::JSX(JSXStatement {
                  jsx: Box::new(JSX::from_element(el)),
                })),
                Expr::JSXFragment(fragment) => stmts.push(Statement::JSX(JSXStatement {
                  jsx: Box::new(JSX::from_fragment(fragment)),
                })),
                _ => stmts.push(Statement::Stmt(Stmt::Labeled(labeled))),
              },
              _ => stmts.push(Statement::Stmt(Stmt::Labeled(labeled))),
            },
            "$style" => match labeled.body.as_ref() {
              Stmt::Expr(ExprStmt { expr, .. }) => match expr.as_ref() {
                Expr::Tpl(tpl) => stmts.push(Statement::Style(StyleStatement {
                  css: Box::new(CSS::parse(tpl)),
                })),
                _ => stmts.push(Statement::Stmt(Stmt::Labeled(labeled))),
              },
              _ => stmts.push(Statement::Stmt(Stmt::Labeled(labeled))),
            },
            _ => {
              let label = labeled.label.as_ref();
              if label.starts_with("$_") {
                stmts.push(Statement::SideEffect(SideEffectStatement {
                  name: Some(label.trim_start_matches("$_").into()),
                  stmt: labeled.body,
                }))
              } else {
                stmts.push(Statement::Stmt(Stmt::Labeled(labeled)))
              }
            }
          },
          _ => stmts.push(Statement::Stmt(stmt)),
        },
      }
    }

    // store the AST to resolver
    resolver.ast = Some(AST { statements: stmts });

    // return a empty moudle
    vec![]
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
