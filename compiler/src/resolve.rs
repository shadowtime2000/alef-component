// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use indexmap::IndexSet;
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
  /// current component specifier
  pub specifier: String,
  /// helper module
  pub helper_module: String,
  /// dependend helpers
  pub dep_helpers: IndexSet<String>,
  /// dependency graph
  pub dep_graph: Vec<DependencyDescriptor>,
  /// inline styles
  pub css: Option<CSSTemplate>,
}

impl Resolver {
  pub fn new(specifier: &str, helper_module: &str) -> Self {
    let mut dep_helpers = IndexSet::<String>::new();
    dep_helpers.insert("Component".into());
    Resolver {
      specifier: specifier.into(),
      helper_module: helper_module.into(),
      dep_helpers,
      dep_graph: Vec::new(),
      css: None,
    }
  }
}

impl Default for Resolver {
  fn default() -> Self {
    let mut dep_helpers = IndexSet::<String>::new();
    dep_helpers.insert("Component".into());
    Resolver {
      specifier: "./App.alef".into(),
      helper_module: "@alephjs/helper".into(),
      dep_helpers,
      dep_graph: Vec::new(),
      css: None,
    }
  }
}
