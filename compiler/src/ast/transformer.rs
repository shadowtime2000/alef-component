// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::{identmap::IdentMap, statement::*, walker::ASTWalker};
use crate::resolve::{format_component_name, Resolver};
use std::{cell::RefCell, path::Path, rc::Rc};
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_utils::quote_ident;
use swc_ecma_visit::{noop_fold_type, Fold};

/// AST Transformer for Alef Component.
pub struct ASTransformer {
  pub resolver: Rc<RefCell<Resolver>>,
}

impl ASTransformer {
  pub fn transform_statements(
    &self,
    scope_idents: IdentMap,
    statements: Vec<Statement>,
  ) -> Vec<Stmt> {
    let mut resolver = self.resolver.borrow_mut();
    let mut stmts: Vec<Stmt> = vec![];

    // insert 'super(props)'
    {
      stmts.push(Stmt::Expr(ExprStmt {
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
      }))
    }

    for stmt in statements {
      match stmt {
        Statement::Var(VarStatement {
          name,
          init,
          is_ref,
          is_array,
          is_async,
        }) => stmts.push(create_swc_stmt(name, init, false)),
        Statement::Const(ConstStatement { name, typed, init }) => match typed {
          ConstTyped::Regular => {
            stmts.push(create_swc_stmt(name, Some(init), true));
          }
          ConstTyped::Memo => {}
          ConstTyped::Prop => {}
          ConstTyped::Slots => {}
          ConstTyped::Context => {}
        },
        Statement::FC(FCStatement {
          scope_idents,
          statements,
        }) => {}
        Statement::SideEffect(SideEffectStatement { name, stmt }) => {}
        Statement::Template(t) => match t {
          TemplateStatement::Element(el) => {}
          TemplateStatement::Fragment(fragment) => {}
        },
        Statement::Style(StyleStatement { css }) => {}
        Statement::Export(ExportStatement { expr }) => {}
        _ => {}
      }
    }

    stmts
  }
}

impl Fold for ASTransformer {
  noop_fold_type!();

  fn fold_module_items(&mut self, items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    let mut walker = ASTWalker::new();
    let statements = walker.walk(items);
    let stmts = self.transform_statements(walker.scope_idents, statements);
    let resolver = self.resolver.borrow();
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

fn create_swc_stmt(name: Pat, init: Option<Box<Expr>>, is_const: bool) -> Stmt {
  Stmt::Decl(Decl::Var(VarDecl {
    span: DUMMY_SP,
    kind: if is_const {
      VarDeclKind::Const
    } else {
      VarDeclKind::Let
    },
    declare: false,
    decls: vec![VarDeclarator {
      span: DUMMY_SP,
      name,
      init,
      definite: false,
    }],
  }))
}
