// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::ast::AST;
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
  /// parsed AST of the component
  pub ast: Option<AST>,
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
      ast: None,
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
      ast: None,
      dep_graph: Vec::new(),
      css: None,
    }
  }
}

pub fn format_component_name(s: &str) -> String {
  let mut should_uppercase = true;
  let mut char_vec: Vec<char> = vec![];
  for c in s.trim_end_matches(".alef").chars() {
    if c >= 'a' && c <= 'z' {
      if should_uppercase {
        should_uppercase = false;
        char_vec.push(c.to_ascii_uppercase());
      } else {
        char_vec.push(c);
      }
    } else if c >= 'A' && c <= 'Z' {
      should_uppercase = false;
      char_vec.push(c);
    } else if (c >= '0' && c <= '9') && char_vec.len() > 0 {
      should_uppercase = false;
      char_vec.push(c);
    } else {
      should_uppercase = true
    }
  }
  if char_vec.len() == 0 {
    return "App".into();
  }
  char_vec.into_iter().collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_format_component_name() {
    assert_eq!(format_component_name("app.alef"), "App");
    assert_eq!(format_component_name("hello-world.alef"), "HelloWorld");
    assert_eq!(format_component_name("hello_world.alef"), "HelloWorld");
    assert_eq!(format_component_name("hello.world.alef"), "HelloWorld");
    assert_eq!(format_component_name("hello world.alef"), "HelloWorld");
    assert_eq!(format_component_name("HELLO world.alef"), "HELLOWorld");
    assert_eq!(format_component_name("h798.alef"), "H798");
    assert_eq!(format_component_name("798hello world.alef"), "HelloWorld");
    assert_eq!(format_component_name("798.alef"), "App");
    assert_eq!(format_component_name("Hello 世界!.alef"), "Hello");
  }
}
