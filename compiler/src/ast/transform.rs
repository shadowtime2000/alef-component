// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::{Statement, AST};
use swc_ecma_ast::*;

/// AST Transform for Alef Component.
pub struct ASTransform {
    pub statements: Vec<Statement>,
}

impl ASTransform {
    pub fn from(ast: &AST) -> Self {
        ASTransform {
            statements: ast.statements.clone(),
        }
    }

    pub fn transform(&mut self) -> Vec<Stmt> {
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
