// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::{identmap::IdentMap, statement::*, walker::ASTWalker};
use crate::resolve::{format_component_name, Resolver};
use indexmap::IndexMap;
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
  ) -> (Vec<Stmt>, IndexMap<String, Option<String>>) {
    let mut resolver = self.resolver.borrow_mut();
    let mut stmts: Vec<Stmt> = vec![];
    let mut dom_helpers: IndexMap<String, Option<String>> = IndexMap::new();

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
        Statement::Var(VarStatement { name, init, .. }) => {
          stmts.push(create_swc_stmt(name, init, false))
        }
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
          TemplateStatement::If(r#if) => {}
        },
        Statement::Style(StyleStatement { css }) => {}
        Statement::Export(ExportStatement { expr }) => {}
        _ => {}
      }
    }

    (stmts, dom_helpers)
  }
}

impl Fold for ASTransformer {
  noop_fold_type!();

  fn fold_module_items(&mut self, items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    let mut walker = ASTWalker::new();
    let statements = walker.walk(items);
    let (stmts, mut dom_helpers) =
      self.transform_statements(walker.scope_idents.clone(), statements);
    let mut resolver = self.resolver.borrow_mut();
    let mut output: Vec<ModuleItem> = vec![];

    let helper_component_id = walker.scope_idents.create_ident("Component");
    dom_helpers.insert("Component".into(), Some(helper_component_id.clone()));

    // import dom helper module
    if resolver.dom_helper_module.starts_with("window.") {
      let mut props: Vec<ObjectPatProp> = vec![];
      for (name, rename) in dom_helpers {
        if let Some(rename) = rename {
          if rename != name {
            props.push(ObjectPatProp::KeyValue(KeyValuePatProp {
              key: PropName::Ident(quote_ident!(name.clone())),
              value: Box::new(Pat::Ident(quote_ident!(rename))),
            }));
            continue;
          }
        }
        props.push(ObjectPatProp::Assign(AssignPatProp {
          span: DUMMY_SP,
          key: quote_ident!(name),
          value: None,
        }));
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
      for (name, rename) in dom_helpers {
        specifiers.push(ImportSpecifier::Named(ImportNamedSpecifier {
          span: DUMMY_SP,
          local: match rename.clone() {
            Some(rename) => {
              if rename != name {
                quote_ident!(rename)
              } else {
                quote_ident!(name.clone())
              }
            }
            _ => quote_ident!(name.clone()),
          },
          imported: match rename {
            Some(rename) => {
              if rename != name {
                Some(quote_ident!(name))
              } else {
                None
              }
            }
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

    // export component class
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
              super_class: Some(Box::new(Expr::Ident(quote_ident!(helper_component_id)))),
              is_abstract: false,
              type_params: None,
              super_type_params: None,
              implements: vec![],
            },
          })),
        },
      )));
    }

    // store dependency graph
    resolver.dep_graph = walker.dep_graph;

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
