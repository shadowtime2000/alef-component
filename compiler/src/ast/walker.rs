// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use crate::resolve::Resolver;
use std::{cell::RefCell, rc::Rc};
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_fold_type, Fold};

/// AST walker for Alef Component.
pub struct ASTWalker {
  pub resolver: Rc<RefCell<Resolver>>,
}

impl Fold for ASTWalker {
  noop_fold_type!();

  fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    vec![]
  }
}
