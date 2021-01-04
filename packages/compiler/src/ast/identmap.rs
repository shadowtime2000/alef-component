// Copyright 2020-2021 postUI Lab. All rights reserved. MIT license.

use indexmap::{IndexMap, IndexSet};
use std::default::Default;
use swc_ecma_ast::*;
use swc_ecma_utils::quote_ident;

pub type IdentSet = IndexSet<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentMap {
    pub helpers: IndexMap<String, String>,
    pub scopes: IdentSet,
    pub states: IdentSet,
    pub array_states: IdentSet,
    pub async_states: IdentSet,
    pub memos: IdentSet,
    pub props: IdentSet,
    pub slotss: IdentSet,
    pub contexts: IdentSet,
}

impl IdentMap {
    fn mark_as(&mut self, name: &str, pat: &Pat) {
        for ident in get_idents_from_pat(&pat) {
            let id = ident.sym.as_ref().to_string();
            match name {
                "scope" => self.scopes.insert(id),
                "state" => self.states.insert(id),
                "array_state" => self.array_states.insert(id),
                "async_state" => self.async_states.insert(id),
                "memo" => self.memos.insert(id),
                "prop" => self.props.insert(id),
                "slots" => self.slotss.insert(id),
                "context" => self.contexts.insert(id),
                _ => false,
            };
        }
    }
    fn mark_as_state(&mut self, pat: &Pat) {
        self.mark(pat);
        self.mark_as("state", pat);
    }
    pub fn mark(&mut self, pat: &Pat) {
        self.mark_as("scope", pat);
    }
    pub fn mark_state(&mut self, pat: &Pat, is_array: bool, is_async: bool) {
        self.mark_as_state(pat);
        if is_array {
            self.mark_as("array_state", pat);
        } else if is_async {
            self.mark_as("async_state", pat);
        }
    }
    pub fn mark_memo(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("memo", pat);
    }
    pub fn mark_prop(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("prop", pat);
    }
    pub fn mark_slots(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("slots", pat);
    }
    pub fn mark_context(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("context", pat);
    }
    pub fn create_ident(&mut self, name: &str) -> Ident {
        let is_helper = match name {
            "Component" | "Element" | "Fragment" | "If" | "List" | "Text" | "Style" | "Memo"
            | "Effect" | "Dirty" | "nope" => true,
            _ => false,
        };
        if is_helper && self.helpers.contains_key(name.into()) {
            return quote_ident!(self.helpers.get(name.into()).unwrap().clone());
        }
        let mut idx = 0;
        if self.scopes.contains(name) {
            idx = 1;
            loop {
                let name = format!("{}{}", name, idx + 1);
                if !self.scopes.contains(&name) {
                    break;
                }
                idx = idx + 1;
            }
        }
        if idx > 0 {
            let fixed_name = format!("{}{}", name, idx + 1);
            self.scopes.insert(fixed_name.clone());
            if is_helper {
                self.helpers.insert(name.into(), fixed_name.clone());
            }
            quote_ident!(fixed_name)
        } else {
            self.scopes.insert(name.into());
            if is_helper {
                self.helpers.insert(name.into(), name.into());
            }
            quote_ident!(name)
        }
    }

    pub fn convert_memo_expr(&self, expr: Expr, deps: &mut Vec<usize>) -> Expr {
        match expr {
            Expr::Ident(id) => {
                if let Some(dep) = self.states.get_index_of(id.sym.as_ref().into()) {
                    deps.push(dep);
                }
                Expr::Ident(id)
            }
            Expr::Update(UpdateExpr {
                span,
                op,
                prefix,
                arg,
            }) => Expr::Update(UpdateExpr {
                span,
                op,
                prefix,
                arg: Box::new(self.convert_dirty_expr(*arg, deps)),
            }),
            Expr::Assign(AssignExpr {
                span,
                op,
                left,
                right,
            }) => Expr::Assign(AssignExpr {
                span,
                op,
                left,
                right: Box::new(self.convert_dirty_expr(*right, deps)),
            }),
            Expr::Paren(ParenExpr {
                span,
                expr: inner_expr,
            }) => Expr::Paren(ParenExpr {
                span,
                expr: Box::new(self.convert_memo_expr(*inner_expr, deps)),
            }),
            Expr::Fn(FnExpr {
                ident,
                function:
                    Function {
                        span,
                        params,
                        decorators,
                        body,
                        is_async,
                        is_generator,
                        type_params,
                        return_type,
                    },
            }) => Expr::Fn(FnExpr {
                ident,
                function: Function {
                    span,
                    params,
                    decorators,
                    body: if let Some(BlockStmt { span, stmts }) = body {
                        Some(BlockStmt {
                            span,
                            stmts: stmts
                                .into_iter()
                                .map(|stmt| match stmt {
                                    Stmt::Expr(ExprStmt { span, expr }) => Stmt::Expr(ExprStmt {
                                        span,
                                        expr: Box::new(self.convert_memo_expr(*expr, deps)),
                                    }),
                                    _ => stmt,
                                })
                                .collect(),
                        })
                    } else {
                        None
                    },
                    is_async,
                    is_generator,
                    type_params,
                    return_type,
                },
            }),
            Expr::Arrow(ArrowExpr {
                span,
                params,
                body,
                is_async,
                is_generator,
                type_params,
                return_type,
                ..
            }) => Expr::Arrow(ArrowExpr {
                span,
                params,
                body: match body {
                    BlockStmtOrExpr::BlockStmt(BlockStmt { span, stmts }) => {
                        BlockStmtOrExpr::BlockStmt(BlockStmt {
                            span,
                            stmts: stmts
                                .into_iter()
                                .map(|stmt| match stmt {
                                    Stmt::Expr(ExprStmt { span, expr }) => Stmt::Expr(ExprStmt {
                                        span,
                                        expr: Box::new(self.convert_memo_expr(*expr, deps)),
                                    }),
                                    _ => stmt,
                                })
                                .collect(),
                        })
                    }
                    BlockStmtOrExpr::Expr(expr) => {
                        BlockStmtOrExpr::Expr(Box::new(self.convert_memo_expr(*expr, deps)))
                    }
                },
                is_async,
                is_generator,
                type_params,
                return_type,
            }),
            _ => expr,
        }
    }

    pub fn convert_dirty_expr(&self, expr: Expr, deps: &mut Vec<usize>) -> Expr {
        match expr {
            Expr::Update(UpdateExpr {
                span,
                op,
                prefix,
                arg,
            }) => {
                if let Expr::Ident(id) = arg.as_ref() {
                    if let Some(dep) = self.states.get_index_of(id.sym.as_ref().into()) {
                        deps.push(dep);
                    }
                }
                Expr::Update(UpdateExpr {
                    span,
                    op,
                    prefix,
                    arg: Box::new(self.convert_dirty_expr(*arg, deps)),
                })
            }
            Expr::Assign(AssignExpr {
                span,
                op,
                left,
                right,
            }) => {
                if let PatOrExpr::Pat(pat) = left.clone() {
                    for id in get_idents_from_pat(&pat) {
                        if let Some(dep) = self.states.get_index_of(id.sym.as_ref().into()) {
                            deps.push(dep);
                        }
                    }
                }
                Expr::Assign(AssignExpr {
                    span,
                    op,
                    left,
                    right: Box::new(self.convert_dirty_expr(*right, deps)),
                })
            }
            Expr::Paren(ParenExpr {
                span,
                expr: inner_expr,
            }) => Expr::Paren(ParenExpr {
                span,
                expr: Box::new(self.convert_dirty_expr(*inner_expr, deps)),
            }),
            Expr::Fn(FnExpr {
                ident,
                function:
                    Function {
                        span,
                        params,
                        decorators,
                        body,
                        is_async,
                        is_generator,
                        type_params,
                        return_type,
                    },
            }) => Expr::Fn(FnExpr {
                ident,
                function: Function {
                    span,
                    params,
                    decorators,
                    body: if let Some(BlockStmt { span, stmts }) = body {
                        Some(BlockStmt {
                            span,
                            stmts: stmts
                                .into_iter()
                                .map(|stmt| match stmt {
                                    Stmt::Expr(ExprStmt { span, expr }) => Stmt::Expr(ExprStmt {
                                        span,
                                        expr: Box::new(self.convert_dirty_expr(*expr, deps)),
                                    }),
                                    _ => stmt,
                                })
                                .collect(),
                        })
                    } else {
                        None
                    },
                    is_async,
                    is_generator,
                    type_params,
                    return_type,
                },
            }),
            Expr::Arrow(ArrowExpr {
                span,
                params,
                body,
                is_async,
                is_generator,
                type_params,
                return_type,
                ..
            }) => Expr::Arrow(ArrowExpr {
                span,
                params,
                body: match body {
                    BlockStmtOrExpr::BlockStmt(BlockStmt { span, stmts }) => {
                        BlockStmtOrExpr::BlockStmt(BlockStmt {
                            span,
                            stmts: stmts
                                .into_iter()
                                .map(|stmt| match stmt {
                                    Stmt::Expr(ExprStmt { span, expr }) => Stmt::Expr(ExprStmt {
                                        span,
                                        expr: Box::new(self.convert_dirty_expr(*expr, deps)),
                                    }),
                                    _ => stmt,
                                })
                                .collect(),
                        })
                    }
                    BlockStmtOrExpr::Expr(expr) => {
                        BlockStmtOrExpr::Expr(Box::new(self.convert_dirty_expr(*expr, deps)))
                    }
                },
                is_async,
                is_generator,
                type_params,
                return_type,
            }),
            _ => expr,
        }
    }
}

impl Default for IdentMap {
    fn default() -> Self {
        IdentMap {
            helpers: IndexMap::new(),
            scopes: IdentSet::new(),
            states: IdentSet::new(),
            array_states: IdentSet::new(),
            async_states: IdentSet::new(),
            memos: IdentSet::new(),
            props: IdentSet::new(),
            slotss: IdentSet::new(),
            contexts: IdentSet::new(),
        }
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
                if let Some(el) = el {
                    for id in get_idents_from_pat(el) {
                        idents.push(id);
                    }
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
