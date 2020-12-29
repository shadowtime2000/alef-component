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
    pub helper_refs: IndexMap<String, u16>,
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
    fn mark_as(&mut self, name: &str, pat: &Pat) {
        for ident in get_idents_from_pat(&pat) {
            let id = ident.sym.as_ref().to_string();
            match name {
                "scope" => {
                    if self.helper_refs.contains_key(&id) {
                        self.helper_refs
                            .insert(id.clone(), self.helper_refs.get(&id).unwrap() + 1);
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
    fn mark_as_state(&mut self, pat: &Pat) {
        self.mark(pat);
        self.mark_as("state", pat);
    }
    pub fn mark(&mut self, pat: &Pat) {
        self.mark_as("scope", pat);
    }
    pub fn mark_state(&mut self, pat: &Pat, is_array: bool, is_async: bool) {
        self.mark_as_state(pat);
        if is_array {
            self.mark_as("array_state", pat);
        } else if is_async {
            self.mark_as("async_state", pat);
        }
    }
    pub fn mark_memo(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("memo", pat);
    }
    pub fn mark_prop(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("prop", pat);
    }
    pub fn mark_slots(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("slots", pat);
    }
    pub fn mark_context(&mut self, pat: &Pat) {
        self.mark_as_state(pat);
        self.mark_as("context", pat);
    }
    pub fn tokenize_helper(&mut self, id: String, refs: u16) {
        self.helper_refs
            .insert(id.clone(), self.helper_refs.get(&id).unwrap() + refs);
    }
    pub fn create_ident(&mut self, name: &str) -> String {
        if self.helper_refs.contains_key(name) {
            let refs = self.helper_refs.get(name).unwrap();
            if *refs > 0 {
                return format!("{}{}", name, refs + 1);
            }
            return name.into();
        }
        let mut idx = 0;
        if self.scopes.contains(name) {
            idx = 1;
            loop {
                let name = format!("{}{}", name, idx + 1);
                if !self.scopes.contains(&name) {
                    break;
                }
                idx = idx + 1;
            }
        }
        if idx > 0 {
            format!("{}{}", name, idx + 1)
        } else {
            name.into()
        }
    }
}

impl Default for IdentMap {
    fn default() -> Self {
        let mut helper_refs = IndexMap::<String, u16>::new();
        for indent in HELPER_IDENTS.iter() {
            helper_refs.insert(indent.to_string(), 0);
        }
        IdentMap {
            helper_refs,
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
