// Copyright 2020-2021 postUI Lab. All rights reserved. MIT license.

use super::{identmap::IdentMap, jsx::JSXTransformer, statement::*, walker::ASTWalker};
use crate::resolve::{to_component_name, Resolver};
use std::{cell::RefCell, iter, path::Path, rc::Rc};
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_utils::{quote_ident, ExprFactory};
use swc_ecma_visit::{noop_fold_type, Fold};

/// AST Transformer for Alef Component.
pub struct ASTransformer {
  pub resolver: Rc<RefCell<Resolver>>,
}

impl Fold for ASTransformer {
  noop_fold_type!();

  fn fold_module_items(&mut self, items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    let mut walker = ASTWalker::new();
    let statements = walker.walk(items);
    let helper_component_id = walker.scope_idents.create_ident("Component");
    let scope_idents = Rc::new(RefCell::new(walker.scope_idents));
    let transformer = StatementsTransformer {
      resolver: self.resolver.clone(),
      scope_idents: scope_idents.clone(),
    };
    let (import_declare, stmts) = transformer.transform(statements);
    let mut resolver = self.resolver.borrow_mut();
    let scope_idents = scope_idents.borrow();
    let mut output: Vec<ModuleItem> = vec![];

    // import runtime module
    if resolver.runtime_module.starts_with("window.") {
      let mut props: Vec<ObjectPatProp> = vec![];
      for (name, rename) in scope_idents.helpers.clone() {
        if rename != name {
          props.push(ObjectPatProp::KeyValue(KeyValuePatProp {
            key: PropName::Ident(quote_ident!(name.clone())),
            value: Box::new(Pat::Ident(quote_ident!(rename))),
          }));
        } else {
          props.push(ObjectPatProp::Assign(AssignPatProp {
            span: DUMMY_SP,
            key: quote_ident!(name),
            value: None,
          }));
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
            prop: quote_ident!(resolver.runtime_module.trim_start_matches("window.")),
          }))),
          definite: false,
        }],
      }))));
    } else {
      let mut specifiers: Vec<ImportSpecifier> = vec![];
      for (name, rename) in scope_idents.helpers.clone() {
        specifiers.push(ImportSpecifier::Named(ImportNamedSpecifier {
          span: DUMMY_SP,
          local: if rename != name {
            quote_ident!(rename.clone())
          } else {
            quote_ident!(name.clone())
          },
          imported: if rename != name {
            Some(quote_ident!(name))
          } else {
            None
          },
        }))
      }
      output.push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers,
        src: Str {
          span: DUMMY_SP,
          value: resolver.runtime_module.as_str().into(),
          has_escape: false,
          kind: Default::default(),
        },
        type_only: false,
        asserts: None,
      })));
    }

    // imports
    for import in import_declare {
      output.push(ModuleItem::ModuleDecl(ModuleDecl::Import(import)));
    }

    // export component class
    {
      let path = Path::new(resolver.specifier.as_str());
      let file_name = path.file_name().as_ref().unwrap().to_str().unwrap();
      let name = to_component_name(file_name);
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
              super_class: Some(Box::new(Expr::Ident(helper_component_id))),
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

pub struct StatementsTransformer {
  pub resolver: Rc<RefCell<Resolver>>,
  pub scope_idents: Rc<RefCell<IdentMap>>,
}

impl StatementsTransformer {
  pub fn transform(&self, statements: Vec<Statement>) -> (Vec<ImportDecl>, Vec<Stmt>) {
    let jsx_transformer = JSXTransformer {
      resolver: self.resolver.clone(),
      scope_idents: self.scope_idents.clone(),
    };
    let mut import_declare: Vec<ImportDecl> = vec![];
    let mut export_default: Option<Expr> = None;
    let mut stmts: Vec<Stmt> = vec![];
    let mut nodes: Vec<Expr> = vec![];

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
        Statement::Import(ImportStatement {
          specifiers, src, ..
        }) => import_declare.push(ImportDecl {
          span: DUMMY_SP,
          specifiers,
          src: Str {
            span: DUMMY_SP,
            value: src.into(),
            has_escape: false,
            kind: Default::default(),
          },
          type_only: false,
          asserts: None,
        }),
        Statement::Var(VarStatement { name, init, .. }) => {
          stmts.push(create_var_decl_stmt(name, init, false))
        }
        Statement::Const(ConstStatement {
          name,
          typed,
          init,
          ctx_name,
        }) => match typed {
          ConstTyped::Regular => {
            stmts.push(create_var_decl_stmt(name, Some(init), true));
          }
          ConstTyped::Memo => {}
          ConstTyped::Prop => {}
          ConstTyped::Slots => {}
          ConstTyped::Context => {}
        },
        Statement::FC(FCStatement {
          name,
          scope_idents,
          statements,
        }) => {}
        Statement::SideEffect(SideEffectStatement { name, stmt }) => {}
        Statement::Template(t) => match t {
          TemplateStatement::Element(el) => {
            nodes.push(jsx_transformer.transform_element(el));
          }
          TemplateStatement::Fragment(frag) => {
            nodes.push(jsx_transformer.transform_fragment(frag));
          }
          TemplateStatement::If(if_stmt) => {
            stmts.push(jsx_transformer.transform_condition(if_stmt));
          }
        },
        Statement::Style(StyleStatement { css }) => {}
        Statement::Export(ExportStatement { expr }) => export_default = Some(expr),
        Statement::Stmt(stmt) => match stmt {
          Stmt::Decl(Decl::Fn(FnDecl {
            ident,
            declare,
            function,
          })) => {
            let fe = Expr::Fn(FnExpr {
              ident: Some(ident.clone()),
              function: function.clone(),
            });
            let mut deps: Vec<usize> = vec![];
            let mut scope_idents = RefCell::borrow_mut(&self.scope_idents);
            scope_idents.convert_dirty_expr(fe.clone(), &mut deps);
            if deps.len() > 0 {
              stmts.push(Stmt::Decl(Decl::Var(VarDecl {
                span: DUMMY_SP,
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![VarDeclarator {
                  span: DUMMY_SP,
                  name: Pat::Ident(ident),
                  init: Some(Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: ExprOrSuper::Expr(Box::new(Expr::Ident(
                      scope_idents.create_ident("Dirty"),
                    ))),
                    args: iter::once(fe.as_arg())
                      .chain(iter::once(
                        Expr::Array(ArrayLit {
                          span: DUMMY_SP,
                          elems: deps
                            .into_iter()
                            .map(|dep| {
                              Some(ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Lit(Lit::Num(Number {
                                  span: DUMMY_SP,
                                  value: dep as f64,
                                }))),
                              })
                            })
                            .collect(),
                        })
                        .as_arg(),
                      ))
                      .collect(),
                    type_args: Default::default(),
                  }))),
                  definite: false,
                }],
              })))
            } else {
              stmts.push(Stmt::Decl(Decl::Fn(FnDecl {
                ident,
                declare,
                function,
              })))
            }
          }
          _ => stmts.push(stmt),
        },
      }
    }

    // const nodes = []
    // this.register(nodes)
    if nodes.len() > 0 {
      let mut scope_idents = RefCell::borrow_mut(&self.scope_idents);
      let nodes_ident = scope_idents.create_ident("nodes");
      stmts.push(Stmt::Decl(Decl::Var(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
          span: DUMMY_SP,
          name: Pat::Ident(nodes_ident.clone()),
          init: Some(Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: nodes
              .into_iter()
              .map(|node| {
                Some(ExprOrSpread {
                  spread: None,
                  expr: Box::new(node),
                })
              })
              .collect(),
          }))),
          definite: false,
        }],
      })));
      stmts.push(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: ExprOrSuper::Expr(Box::new(Expr::This(ThisExpr { span: DUMMY_SP }))),
          prop: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: ExprOrSuper::Expr(Box::new(Expr::Ident(quote_ident!("register")))),
            args: vec![ExprOrSpread {
              spread: None,
              expr: Box::new(Expr::Ident(nodes_ident.clone())),
            }],
            type_args: None,
          })),
          computed: false,
        })),
      }))
    }

    (import_declare, stmts)
  }
}

fn create_var_decl_stmt(name: Pat, init: Option<Expr>, is_const: bool) -> Stmt {
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
      init: if let Some(init) = init {
        Some(Box::new(init))
      } else {
        None
      },
      definite: false,
    }],
  }))
}
