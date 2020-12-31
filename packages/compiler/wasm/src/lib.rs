use alef::compiler::Compiler;
use alef::resolve::{CSSTemplate, DependencyDescriptor, Resolver, Target};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CompileOptions {
  #[serde(default = "default_runtime_module")]
  pub runtime_module: String,

  #[serde(default = "default_target")]
  pub target: Target,

  #[serde(default)]
  pub is_dev: bool,

  #[serde(default)]
  pub hot_refresh: bool,
}

fn default_runtime_module() -> String {
  "alef-dom".into()
}

fn default_target() -> Target {
  Target::Es2020
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformOutput {
  pub code: String,
  pub deps: Vec<DependencyDescriptor>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub css: Option<CSSTemplate>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub map: Option<String>,
}

#[wasm_bindgen(js_name = "transformSync")]
pub fn transform_sync(specifier: &str, source: &str, opts: JsValue) -> Result<JsValue, JsValue> {
  console_error_panic_hook::set_once();

  let opts: CompileOptions = opts
    .into_serde()
    .map_err(|err| format!("failed to parse options: {}", err))
    .unwrap();
  let resolver = Rc::new(RefCell::new(Resolver::new(
    specifier,
    opts.runtime_module.as_str(),
  )));
  let compiler = Compiler::parse(specifier, source).expect("could not parse module");
  let (code, map) = compiler
    .transpile(resolver.clone())
    .expect("could not transpile module");
  let r = resolver.borrow_mut();
  Ok(
    JsValue::from_serde(&TransformOutput {
      code,
      map,
      deps: r.dep_graph.clone(),
      css: r.css.clone(),
    })
    .unwrap(),
  )
}
