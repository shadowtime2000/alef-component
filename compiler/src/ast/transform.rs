// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::{Statement, AST};
use swc_ecma_ast::*;

/// AST Transform for Alef Component.
pub struct ASTransform {
    pub statements: Vec<Statement>,
}

impl ASTransform {
    pub fn new(ast: &AST) -> Self {
        ASTransform {
            statements: ast.statements.clone(),
        }
    }

    pub fn transform(&mut self) -> Vec<Stmt> {
        vec![]
    }
}
