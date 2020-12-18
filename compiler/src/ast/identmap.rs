// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use indexmap::{IndexMap, IndexSet};
use std::default::Default;
use swc_ecma_ast::*;

const HELPER_IDENTS: [&'static str; 14] = [
    "Component",
    "New",
    "Element",
    "Fragment",
    "If",
    "IfElse",
    "List",
    "Text",
    "Space",
    "Style",
    "Memo",
    "Effect",
    "banchUpdate",
    "nope",
];

pub type IdentSet = IndexSet<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentMap {
    pub helpers: IndexMap<String, u16>,
    pub scopes: IdentSet,
    pub states: IdentSet,
    pub array_states: IdentSet,
    pub async_states: IdentSet,
    pub memos: IdentSet,
    pub props: IdentSet,
    pub slotss: IdentSet,
    pub contexts: IdentSet,
}

impl IdentMap {
    fn add_to(&mut self, name: &str, pat: &Pat) {
        for ident in get_idents_from_pat(&pat) {
            let id = ident.sym.as_ref().to_string();
            match name {
                "scope" => {
                    if self.helpers.contains_key(&id) {
                        self.helpers
                            .insert(id.clone(), self.helpers.get(&id).unwrap() + 1);
                    }
                    self.scopes.insert(id)
                }
                "state" => self.states.insert(id),
                "array_state" => self.array_states.insert(id),
                "async_state" => self.async_states.insert(id),
                "memo" => self.memos.insert(id),
                "prop" => self.props.insert(id),
                "slots" => self.slotss.insert(id),
                "context" => self.contexts.insert(id),
                _ => false,
            };
        }
    }
    fn add_to_states(&mut self, pat: &Pat) {
        self.add(pat);
        self.add_to("states", pat);
    }
    pub fn add_helper_refs(&mut self, id: String, n: u16) {
        self.helpers
            .insert(id.clone(), self.helpers.get(&id).unwrap() + n);
    }
    pub fn add(&mut self, pat: &Pat) {
        self.add_to("scopes", pat);
    }
    pub fn add_state(&mut self, pat: &Pat, is_array: bool, is_async: bool) {
        self.add_to_states(pat);
        if is_array {
            self.add_to("array_states", pat);
        } else if is_async {
            self.add_to("async_states", pat);
        }
    }
    pub fn add_memo(&mut self, pat: &Pat) {
        self.add_to_states(pat);
        self.add_to("memos", pat);
    }
    pub fn add_prop(&mut self, pat: &Pat) {
        self.add_to_states(pat);
        self.add_to("props", pat);
    }
    pub fn add_slots(&mut self, pat: &Pat) {
        self.add_to_states(pat);
        self.add_to("slotss", pat);
    }
    pub fn add_context(&mut self, pat: &Pat) {
        self.add_to_states(pat);
        self.add_to("contexts", pat);
    }
}

impl Default for IdentMap {
    fn default() -> Self {
        let mut helpers = IndexMap::<String, u16>::new();
        for indent in HELPER_IDENTS.iter() {
            helpers.insert(indent.to_string(), 0);
        }
        IdentMap {
            helpers,
            scopes: IdentSet::new(),
            states: IdentSet::new(),
            array_states: IdentSet::new(),
            async_states: IdentSet::new(),
            memos: IdentSet::new(),
            props: IdentSet::new(),
            slotss: IdentSet::new(),
            contexts: IdentSet::new(),
        }
    }
}

fn get_idents_from_pat(pat: &Pat) -> Vec<Ident> {
    let mut idents: Vec<Ident> = vec![];
    match pat {
        Pat::Ident(id) => {
            idents.push(id.clone());
        }
        Pat::Array(ArrayPat { elems, .. }) => {
            for el in elems {
                if let Some(el) = el {
                    for id in get_idents_from_pat(el) {
                        idents.push(id);
                    }
                }
            }
        }
        Pat::Object(ObjectPat { props, .. }) => {
            for prop in props {
                match prop {
                    ObjectPatProp::Assign(AssignPatProp { key, .. }) => idents.push(key.clone()),
                    ObjectPatProp::KeyValue(KeyValuePatProp { value, .. }) => {
                        for id in get_idents_from_pat(value.as_ref()) {
                            idents.push(id)
                        }
                    }
                    ObjectPatProp::Rest(RestPat { arg, .. }) => {
                        for id in get_idents_from_pat(arg.as_ref()) {
                            idents.push(id)
                        }
                    }
                }
            }
        }
        _ => {}
    };
    idents
}
