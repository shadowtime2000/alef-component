mod css;
mod jsx;
mod statement;
mod transform;
mod walker;

use crate::resolve::Resolver;
use css::CSS;
use jsx::JSX;
use statement::Statement;
use std::{cell::RefCell, rc::Rc};
use swc_ecma_ast::*;
use swc_ecma_visit::Fold;
use transform::ASTransform;
use walker::ASTWalker;

/// AST for Alef Component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AST {
  pub statements: Vec<Statement>,
}

pub fn ast_walker(resolver: Rc<RefCell<Resolver>>) -> impl Fold {
  ASTWalker {
    resolver: resolver.clone(),
  }
}

pub fn ast_trasnform(ast: &AST) -> Vec<Stmt> {
  ASTransform::from(ast).transform()
}
