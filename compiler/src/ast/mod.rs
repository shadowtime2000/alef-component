mod css;
mod statement;
mod transform;
mod walker;

use crate::resolve::Resolver;
use statement::Statement;
use std::{cell::RefCell, rc::Rc};
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

pub fn ast_transform(resolver: Rc<RefCell<Resolver>>) -> impl Fold {
  ASTransform {
    resolver: resolver.clone(),
  }
}
