// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use crate::resolve::Resolver;
use std::{cell::RefCell, rc::Rc};
use swc_common::{SourceMap, Spanned, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_utils::quote_ident;
use swc_ecma_visit::{noop_fold_type, Fold};

pub fn ast_walker(
    resolver: Rc<RefCell<Resolver>>,
    source: Rc<SourceMap>,
    is_dev: bool,
) -> impl Fold {
    ASTWalker {
        resolver: resolver.clone(),
        source: source.clone(),
        is_dev:is_dev,
    }
}

/// AST wakler for Alef Component.
struct ASTWalker {
    resolver: Rc<RefCell<Resolver>>,
    source: Rc<SourceMap>,
    is_dev: bool,
}

impl Fold for ASTWalker {
    noop_fold_type!();

    fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
        module_items
    }
}
