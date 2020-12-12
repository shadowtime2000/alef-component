// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyDescriptor {
  /// The text specifier associated with the import/export statement.
  pub specifier: String,
  /// A flag indicating if the import is dynamic or not.
  pub is_dynamic: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CSSTemplate {
  pub quasis: Vec<String>,
  pub exprs: Vec<String>,
}

/// A Resolver to resolve aleph.js import/export URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resolver {
  /// dependency graph
  pub dep_graph: Vec<DependencyDescriptor>,
  /// inline styles
  pub css:  Option<CSSTemplate>,
}

impl Resolver {
  pub fn new() -> Self {
    Resolver {
      dep_graph: Vec::new(),
      css: None,
    }
  }
}
