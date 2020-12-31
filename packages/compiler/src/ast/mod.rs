// Copyright 2020-2021 postUI Lab. All rights reserved. MIT license.

mod css;
mod identmap;
mod jsx;
mod statement;
mod transformer;
mod walker;

use crate::resolve::Resolver;
use std::{cell::RefCell, rc::Rc};
use swc_ecma_visit::Fold;
use transformer::ASTransformer;

pub fn alef_transform(resolver: Rc<RefCell<Resolver>>) -> impl Fold {
  ASTransformer {
    resolver: resolver.clone(),
  }
}
