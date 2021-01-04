// Copyright 2020-2021 postUI Lab. All rights reserved. MIT license.

use super::{css::CSS, identmap::IdentMap, statement::*};
use crate::resolve::DependencyDescriptor;
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;

/// AST walker for Alef Component.
pub struct ASTWalker {
  pub dep_graph: Vec<DependencyDescriptor>,
  pub scope_idents: IdentMap,
}

impl ASTWalker {
  pub fn new() -> Self {
    ASTWalker {
      dep_graph: vec![],
      scope_idents: IdentMap::default(),
    }
  }

  /// transform `swc_ecma_ast::Stmt` to `Vec<Statement>`
  fn transform_stmt(&mut self, stmt: &Stmt) -> Vec<Statement> {
    let mut stmts: Vec<Statement> = vec![];

    match stmt {
      Stmt::Decl(Decl::Var(VarDecl { kind, decls, .. })) => match kind {
        VarDeclKind::Const => {
          for decl in decls {
            let mut typed = ConstTyped::Regular;
            let mut ctx_name: Option<String> = None;
            if let Pat::Ident(Ident { ref type_ann, .. })
            | Pat::Array(ArrayPat { ref type_ann, .. })
            | Pat::Object(ObjectPat { ref type_ann, .. }) = decl.name
            {
              if let Some(TsTypeAnn { type_ann, .. }) = type_ann {
                if let TsType::TsTypeRef(TsTypeRef {
                  type_name: TsEntityName::Ident(Ident { sym, .. }),
                  type_params,
                  ..
                }) = type_ann.as_ref()
                {
                  match sym.as_ref() {
                    "Prop" => {
                      typed = ConstTyped::Prop;
                      if let Some(type_params) = type_params {
                        if let Some(param) = type_params.params.first() {
                          if let TsType::TsTypeRef(TsTypeRef {
                            type_name: TsEntityName::Ident(Ident { sym, .. }),
                            type_params: None,
                            ..
                          }) = param.as_ref()
                          {
                            if sym.eq("Slots") {
                              typed = ConstTyped::Slots
                            }
                          }
                        }
                      }
                    }
                    "Context" => {
                      typed = ConstTyped::Context;
                      if let Some(type_params) = type_params {
                        if let Some(param) = type_params.params.first() {
                          if let TsType::TsLitType(TsLitType {
                            lit: TsLit::Str(Str { value, .. }),
                            ..
                          }) = param.as_ref()
                          {
                            ctx_name = Some(value.as_ref().into())
                          }
                        }
                      }
                    }
                    "Memo" => typed = ConstTyped::Memo,
                    "FC" => {
                      if let Pat::Ident(name) = &decl.name {
                        if name.sym.chars().next().unwrap().is_ascii_uppercase() {
                          if let Some(init) = &decl.init {
                            match init.as_ref() {
                              Expr::Arrow(ArrowExpr { body, .. }) => match body {
                                BlockStmtOrExpr::BlockStmt(block_stmt) => {
                                  let mut fc_walker = Self::new();
                                  let mut fc_stmts: Vec<Statement> = vec![];
                                  for stmt in &block_stmt.stmts {
                                    fc_stmts = [fc_stmts, fc_walker.transform_stmt(stmt)].concat()
                                  }
                                  for dep in fc_walker.dep_graph {
                                    self.dep_graph.push(dep)
                                  }
                                  self.scope_idents.mark(&decl.name);
                                  stmts.push(Statement::FC(FCStatement {
                                    name: name.clone(),
                                    scope_idents: fc_walker.scope_idents,
                                    statements: fc_stmts,
                                  }));
                                  continue;
                                }
                                BlockStmtOrExpr::Expr(expr) => {
                                  let mut fc_walker = Self::new();
                                  let statements =
                                    fc_walker.transform_stmt(&Stmt::Expr(ExprStmt {
                                      span: DUMMY_SP,
                                      expr: expr.clone(),
                                    }));
                                  for dep in fc_walker.dep_graph {
                                    self.dep_graph.push(dep)
                                  }
                                  self.scope_idents.mark(&decl.name);
                                  stmts.push(Statement::FC(FCStatement {
                                    name: name.clone(),
                                    scope_idents: fc_walker.scope_idents,
                                    statements,
                                  }));
                                  continue;
                                }
                              },
                              Expr::Fn(FnExpr {
                                function:
                                  Function {
                                    body: Some(body),
                                    is_generator: false,
                                    ..
                                  },
                                ..
                              }) => {
                                let mut fc_walker = Self::new();
                                let mut fc_stmts: Vec<Statement> = vec![];
                                for stmt in &body.stmts {
                                  fc_stmts = [fc_stmts, fc_walker.transform_stmt(stmt)].concat()
                                }
                                for dep in fc_walker.dep_graph {
                                  self.dep_graph.push(dep)
                                }
                                self.scope_idents.mark(&decl.name);
                                stmts.push(Statement::FC(FCStatement {
                                  name: name.clone(),
                                  scope_idents: fc_walker.scope_idents,
                                  statements: fc_stmts,
                                }));
                                continue;
                              }
                              _ => {}
                            }
                          }
                        }
                      }
                    }
                    _ => {}
                  }
                }
              }
            }
            match typed {
              ConstTyped::Regular => self.scope_idents.mark(&decl.name),
              ConstTyped::Memo => self.scope_idents.mark_memo(&decl.name),
              ConstTyped::Prop => self.scope_idents.mark_prop(&decl.name),
              ConstTyped::Slots => self.scope_idents.mark_slots(&decl.name),
              ConstTyped::Context => self.scope_idents.mark_context(&decl.name),
            }
            stmts.push(Statement::Const(ConstStatement {
              typed,
              name: decl.name.clone(),
              init: decl.init.clone().unwrap().as_ref().clone(),
              ctx_name,
            }))
          }
        }
        _ => {
          for decl in decls {
            let mut is_array = false;
            let mut is_ref = false;
            let is_async = match decl.init {
              Some(ref init) => match init.as_ref() {
                Expr::Await(_) => true,
                _ => false,
              },
              _ => false,
            };
            if let Pat::Ident(Ident { ref type_ann, .. })
            | Pat::Array(ArrayPat { ref type_ann, .. })
            | Pat::Object(ObjectPat { ref type_ann, .. }) = decl.name
            {
              if let Some(TsTypeAnn { type_ann, .. }) = type_ann {
                match type_ann.as_ref() {
                  TsType::TsArrayType(_) => is_array = true,
                  TsType::TsTypeRef(TsTypeRef {
                    type_name: TsEntityName::Ident(Ident { sym, .. }),
                    ..
                  }) => is_ref = sym.eq("Ref"),
                  _ => {}
                }
              }
            }
            // todo: check `let a = [1, 2, 3]` get array definite
            if is_ref {
              self.scope_idents.mark(&decl.name)
            } else {
              self.scope_idents.mark_state(&decl.name, is_array, is_async)
            }
            stmts.push(Statement::Var(VarStatement {
              name: decl.name.clone(),
              init: if let Some(init) = &decl.init {
                Some(init.as_ref().clone())
              } else {
                None
              },
              is_array,
              is_ref,
              is_async,
            }))
          }
        }
      },
      Stmt::Expr(ExprStmt { expr, .. }) => {
        if let Expr::Fn(FnExpr {
          ident: Some(ident), ..
        }) = expr.as_ref()
        {
          self.scope_idents.mark(&Pat::Ident(ident.clone()));
        }
        stmts.push(Statement::Stmt(stmt.clone()));
      }
      Stmt::Labeled(labeled) => match labeled.label.as_ref() {
        "$" => stmts.push(Statement::SideEffect(SideEffectStatement {
          name: None,
          stmt: labeled.body.as_ref().clone(),
        })),
        "$t" => match labeled.body.as_ref() {
          Stmt::Expr(ExprStmt { expr, .. }) => match expr.as_ref() {
            // match `$t: <p />`
            Expr::JSXElement(el) => stmts.push(Statement::Template(TemplateStatement::Element(
              el.as_ref().clone(),
            ))),
            // match `$t: <><p /></>`
            Expr::JSXFragment(fragment) => stmts.push(Statement::Template(
              TemplateStatement::Fragment(fragment.clone()),
            )),
            // match `$t: true ? <p /> : <p />`
            Expr::Cond(CondExpr {
              test, cons, alt, ..
            }) => stmts.push(Statement::Template(TemplateStatement::If(IfStmt {
              span: DUMMY_SP,
              test: test.clone(),
              cons: Box::new(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: cons.clone(),
              })),
              alt: Some(Box::new(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: alt.clone(),
              }))),
            }))),
            Expr::Bin(BinExpr {
              op, left, right, ..
            }) => match op {
              // match `$t: true && <p />` OR `$t: true && 1 && <p />`
              BinaryOp::LogicalAnd => {
                stmts.push(Statement::Template(TemplateStatement::If(IfStmt {
                  span: DUMMY_SP,
                  test: left.clone(),
                  cons: Box::new(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: right.clone(),
                  })),
                  alt: None,
                })))
              }
              // match `$t: false || <p />` OR `$t: false || 0 || <p />`
              BinaryOp::LogicalOr => {
                stmts.push(Statement::Template(TemplateStatement::If(IfStmt {
                  span: DUMMY_SP,
                  test: Box::new(Expr::Unary(UnaryExpr {
                    span: DUMMY_SP,
                    op: UnaryOp::Bang,
                    arg: left.clone(),
                  })),
                  cons: Box::new(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: right.clone(),
                  })),
                  alt: None,
                })))
              }
              _ => stmts.push(Statement::Stmt(stmt.clone())),
            },
            _ => stmts.push(Statement::Stmt(stmt.clone())),
          },
          // match `$t: if (...) <p /> else if (...) <p /> else <p />`
          Stmt::If(IfStmt {
            test, cons, alt, ..
          }) => stmts.push(Statement::Template(TemplateStatement::If(IfStmt {
            span: DUMMY_SP,
            test: test.clone(),
            cons: cons.clone(),
            alt: alt.clone(),
          }))),
          _ => stmts.push(Statement::Stmt(stmt.clone())),
        },
        "$style" => match labeled.body.as_ref() {
          Stmt::Expr(ExprStmt { expr, .. }) => match expr.as_ref() {
            Expr::Tpl(tpl) => stmts.push(Statement::Style(StyleStatement {
              css: CSS::parse(tpl),
            })),
            _ => stmts.push(Statement::Stmt(stmt.clone())),
          },
          _ => stmts.push(Statement::Stmt(stmt.clone())),
        },
        _ => {
          let label = labeled.label.as_ref();
          if label.starts_with("$_") {
            stmts.push(Statement::SideEffect(SideEffectStatement {
              name: Some(label.trim_start_matches("$_").into()),
              stmt: labeled.body.as_ref().clone(),
            }));
          } else {
            stmts.push(Statement::Stmt(stmt.clone()));
          }
        }
      },
      Stmt::Return(ReturnStmt {
        arg: Some(expr), ..
      }) => match expr.as_ref() {
        Expr::JSXElement(el) => stmts.push(Statement::Template(TemplateStatement::Element(
          el.as_ref().clone(),
        ))),
        Expr::JSXFragment(fragment) => stmts.push(Statement::Template(
          TemplateStatement::Fragment(fragment.clone()),
        )),
        _ => stmts.push(Statement::Stmt(stmt.clone())),
      },
      _ => stmts.push(Statement::Stmt(stmt.clone())),
    };

    stmts
  }

  pub fn walk(&mut self, module_items: Vec<ModuleItem>) -> Vec<Statement> {
    let mut stmts: Vec<Statement> = vec![];

    for item in module_items {
      match item {
        ModuleItem::ModuleDecl(decl) => match decl {
          ModuleDecl::Import(ImportDecl {
            specifiers, src, ..
          }) => {
            let src = src.value.as_ref();
            for specifier in specifiers.clone() {
              if let ImportSpecifier::Default(ImportDefaultSpecifier { local, .. })
              | ImportSpecifier::Named(ImportNamedSpecifier { local, .. }) = specifier
              {
                self.scope_idents.mark(&Pat::Ident(local))
              }
            }
            self.dep_graph.push(DependencyDescriptor {
              specifier: src.into(),
              is_dynamic: false,
            });
            stmts.push(Statement::Import(ImportStatement {
              specifiers,
              src: src.into(),
              is_alef_component: src.ends_with(".alef"),
            }))
          }
          ModuleDecl::ExportDefaultExpr(ExportDefaultExpr { expr, .. }) => {
            stmts.push(Statement::Export(ExportStatement {
              expr: expr.as_ref().clone(),
            }))
          }
          _ => {}
        },
        ModuleItem::Stmt(ref stmt) => stmts = [stmts, self.transform_stmt(stmt)].concat(),
      }
    }

    stmts
  }
}
