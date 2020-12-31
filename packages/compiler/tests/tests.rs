// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

mod common;

use common::{t, t_custom_runtime_module};

#[test]
fn test_dom_helper_module() {
  let (code, _) = t_custom_runtime_module("./App.alef", "", "https://deno.land/x/alef/dom.ts");
  assert!(code.contains(" from \"https://deno.land/x/alef/dom.ts\";"));
  let (code, _) = t_custom_runtime_module("./App.alef", "", "window.__ALEF_DOM");
  assert!(code.contains("} = window.__ALEF_DOM;"));
}

#[test]
fn test_component_export() {
  let source = r#"
    let name: string = 'World'

    $t: <p>Hello {name}!</p>    
  "#;
  let (code, _) = t("App.alef", source);
  assert!(code.contains("import { Component, Element, Memo } from \"alef-dom\";"));
  assert!(code.contains("export default class App extends Component"));
  assert!(code.contains("constructor(props)"));
  assert!(code.contains("super(props)"));
  assert!(code.contains("const nodes = ["));
  assert!(code.contains("this.register(nodes)"));
}
