// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use swc_ecma_ast::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JSX {}

impl JSX {
  pub fn from_element(el: &JSXElement) -> Self {
    JSX {}
  }

  pub fn from_fragment(fragment: &JSXFragment) -> Self {
    JSX {}
  }
}
