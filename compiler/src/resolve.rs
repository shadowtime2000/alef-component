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
  /// current component specifier
  pub specifier: String,
  /// dom helper module
  pub dom_helper_module: String,
  /// dependency graph
  pub dep_graph: Vec<DependencyDescriptor>,
  /// inline styles
  pub css: Option<CSSTemplate>,
}

impl Resolver {
  pub fn new(specifier: &str, dom_helper_module: &str) -> Self {
    Resolver {
      specifier: specifier.into(),
      dom_helper_module: dom_helper_module.into(),
      dep_graph: Vec::new(),
      css: None,
    }
  }
}

impl Default for Resolver {
  fn default() -> Self {
    Resolver {
      specifier: "./App.alef".into(),
      dom_helper_module: "alef-dom".into(),
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
