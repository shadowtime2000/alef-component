mod ast;
mod css;
mod jsx;
mod walker;

use crate::resolve::Resolver;
use std::{cell::RefCell, rc::Rc};
use swc_ecma_visit::Fold;
use walker::ASTWalker;

pub fn ast_walker(resolver: Rc<RefCell<Resolver>>) -> impl Fold {
  ASTWalker {
    resolver: resolver.clone(),
  }
}
