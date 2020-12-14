// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

mod ast;
mod code_gen;
mod error;
mod module;
mod resolve;

use module::AlefComponentModule;
use resolve::{CSSTemplate, DependencyDescriptor, Resolver};
use serde::Deserialize;
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};
use swc_ecmascript::parser::JscTarget;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CompileOptions {
  #[serde(default = "default_dom_helper_module")]
  pub dom_helper_module: String,

  #[serde(default = "default_target")]
  pub target: JscTarget,

  #[serde(default)]
  pub is_dev: bool,

  #[serde(default)]
  pub hot_refresh: bool,
}

fn default_dom_helper_module() -> String {
  "alef-dom".into()
}

fn default_target() -> JscTarget {
  JscTarget::Es2020
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
    opts.dom_helper_module.as_str(),
  )));
  let module = AlefComponentModule::parse(specifier, source).expect("could not parse module");
  let (code, map) = module
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
