// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use swc_ecma_ast::*;
use cssparser::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CSS {}

impl CSS {
  pub fn parse(tpl: &Tpl) -> Self {
    // I should probably be looping over quasis values or something but I have no exact idea on what to do here
    let quasis_first = tpl.quasis.first();
    match quasis_first {
      Some(p) => {
        let input = &mut ParserInput::new(p.raw.value.as_ref());
        let parser = &mut Parser::new(input);
        CSS {}
      },
      None => CSS {}
    }
  }
}
