// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

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
                    "Context" => typed = ConstTyped::Context,
                    "Memo" => typed = ConstTyped::Memo,
                    "FC" => {
                      if let Some(init) = &decl.init {
                        match init.as_ref() {
                          Expr::Arrow(ArrowExpr { body, .. }) => match body {
                            BlockStmtOrExpr::BlockStmt(block_stmt) => {
                              let mut fc_walker = Self::new();
                              let mut fc_stmts: Vec<Statement> = vec![];
                              for stmt in &block_stmt.stmts {
                                fc_stmts = [fc_stmts, fc_walker.transform_stmt(stmt)].concat()
                              }
                              for (id, refs) in fc_walker.scope_idents.helper_refs.clone() {
                                self.scope_idents.tokenize_helper(id, refs)
                              }
                              for dep in fc_walker.dep_graph {
                                self.dep_graph.push(dep)
                              }
                              self.scope_idents.mark(&decl.name);
                              stmts.push(Statement::FC(FCStatement {
                                scope_idents: fc_walker.scope_idents,
                                statements: fc_stmts,
                              }));
                              continue;
                            }
                            BlockStmtOrExpr::Expr(expr) => {
                              let mut fc_walker = Self::new();
                              let statements = fc_walker.transform_stmt(&Stmt::Expr(ExprStmt {
                                span: DUMMY_SP,
                                expr: expr.clone(),
                              }));
                              for (id, refs) in fc_walker.scope_idents.helper_refs.clone() {
                                self.scope_idents.tokenize_helper(id, refs)
                              }
                              for dep in fc_walker.dep_graph {
                                self.dep_graph.push(dep)
                              }
                              self.scope_idents.mark(&decl.name);
                              stmts.push(Statement::FC(FCStatement {
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
                            for (id, refs) in fc_walker.scope_idents.helper_refs.clone() {
                              self.scope_idents.tokenize_helper(id, refs)
                            }
                            for dep in fc_walker.dep_graph {
                              self.dep_graph.push(dep)
                            }
                            self.scope_idents.mark(&decl.name);
                            stmts.push(Statement::FC(FCStatement {
                              scope_idents: fc_walker.scope_idents,
                              statements: fc_stmts,
                            }));
                            continue;
                          }
                          _ => {}
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
              init: decl.init.clone().unwrap(),
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
              init: decl.init.clone(),
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
          stmt: labeled.body.clone(),
        })),
        "$t" => match labeled.body.as_ref() {
          Stmt::Expr(ExprStmt { expr, .. }) => match expr.as_ref() {
            Expr::JSXElement(el) => stmts.push(Statement::Template(TemplateStatement::Element(
              el.as_ref().clone(),
            ))),
            Expr::JSXFragment(fragment) => stmts.push(Statement::Template(
              TemplateStatement::Fragment(fragment.clone()),
            )),
            _ => stmts.push(Statement::Stmt(stmt.clone())),
          },
          _ => stmts.push(Statement::Stmt(stmt.clone())),
        },
        "$style" => match labeled.body.as_ref() {
          Stmt::Expr(ExprStmt { expr, .. }) => match expr.as_ref() {
            Expr::Tpl(tpl) => stmts.push(Statement::Style(StyleStatement {
              css: Box::new(CSS::parse(tpl)),
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
              stmt: labeled.body.clone(),
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
            stmts.push(Statement::Export(ExportStatement { expr }))
          }
          _ => {}
        },
        ModuleItem::Stmt(ref stmt) => stmts = [stmts, self.transform_stmt(stmt)].concat(),
      }
    }

    stmts
  }
}
