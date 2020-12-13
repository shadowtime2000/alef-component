// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::*;
use crate::resolve::Resolver;
use std::{cell::RefCell, rc::Rc};
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_fold_type, Fold};

/// AST walker for Alef Component.
pub struct ASTWalker {
  pub resolver: Rc<RefCell<Resolver>>,
}

impl Fold for ASTWalker {
  noop_fold_type!();

  fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    let mut resolver = self.resolver.borrow_mut();
    let mut stmts: Vec<Statement> = vec![];

    for item in module_items {
      match item {
        ModuleItem::ModuleDecl(decl) => {}
        ModuleItem::Stmt(stmt) => match stmt {
          Stmt::Decl(Decl::Var(VarDecl { kind, decls, .. })) => match kind {
            VarDeclKind::Const => {
              for decl in decls {

              }
            }
            _ => {
              for decl in decls {
                
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
                  jsx: Box::new(JSX {}),
                })),
                Expr::JSXFragment(fragment) => {}
                Expr::JSXNamespacedName(name) => {}
                _ => stmts.push(Statement::Stmt(Stmt::Labeled(labeled))),
              },
              _ => stmts.push(Statement::Stmt(Stmt::Labeled(labeled))),
            },
            "$style" => match labeled.body.as_ref() {
              Stmt::Expr(ExprStmt { expr, .. }) => match expr.as_ref() {
                Expr::Tpl(tpl) => stmts.push(Statement::Style(StyleStatement {
                  css: Box::new(CSS {}),
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

    // store ast
    resolver.ast = Some(AST { statements: stmts });

    vec![]
  }
}
