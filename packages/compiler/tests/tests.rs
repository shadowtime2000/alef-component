mod common;

use common::{t, t_custom_runtime_module};
use regex::Regex;

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

#[test]
fn test_component_jsx() {
  let source = r#"
    let n = 0

    $t: <p>current count is <strong>{n}</strong></p>
    $t: <button onClick={() => { n-- }}>-</button>
    $t: <button onClick={() => { n++ }}>+</button>
  "#;
  let (code, _) = t("App.alef", source);
  assert!(code.contains("import { Component, Element, Memo, Dirty } from \"alef-dom\";"));
  assert!(code.contains("Element(\"p\", null, \"current count is \", Element(\"strong\", null"));
  assert!(code.contains("Element(\"button\", {"));

  let r = Regex::new(r"Memo\(\(\)\s*=>\s*n\s*,\s*\[\s*0\s*\]\)").unwrap();
  assert!(r.is_match(code.as_str()));
  let r =
    Regex::new(r"onClick:\s*Dirty\(\(\)\s*=>\s*\{\s*n(\+\+|--);?\s*\}\s*,\s*\[\s*0\s*\]\s*\)")
      .unwrap();
  assert!(r.is_match(code.as_str()));
}
