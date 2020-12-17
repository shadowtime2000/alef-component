// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use indexmap::IndexSet;
use std::default::Default;
use swc_ecma_ast::*;

pub type IdentSet = IndexSet<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentMap {
    pub scope: IdentSet,
    pub state: IdentSet,
    pub array_state: IdentSet,
    pub async_state: IdentSet,
    pub memo: IdentSet,
    pub prop: IdentSet,
    pub slots: IdentSet,
    pub context: IdentSet,
}

impl IdentMap {
    fn add_to(&mut self, name: &str, pat: &Pat) {
        for ident in get_idents_from_pat(&pat) {
            let id = ident.sym.as_ref().to_string();
            match name {
                "scope" => self.scope.insert(id),
                "state" => self.state.insert(id),
                "array_state" => self.array_state.insert(id),
                "async_state" => self.async_state.insert(id),
                "memo" => self.memo.insert(id),
                "prop" => self.prop.insert(id),
                "slots" => self.slots.insert(id),
                "context" => self.context.insert(id),
                _ => false,
            };
        }
    }
    fn add_to_state(&mut self, pat: &Pat) {
        self.add(pat);
        self.add_to("state", pat);
    }
    pub fn add(&mut self, pat: &Pat) {
        self.add_to("scope", pat);
    }
    pub fn add_state(&mut self, pat: &Pat, is_array: bool, is_async: bool) {
        self.add_to_state(pat);
        if is_array {
            self.add_to("array_state", pat);
        } else if is_async {
            self.add_to("async_state", pat);
        }
    }
    pub fn add_memo(&mut self, pat: &Pat) {
        self.add_to_state(pat);
        self.add_to("memo", pat);
    }
    pub fn add_prop(&mut self, pat: &Pat) {
        self.add_to_state(pat);
        self.add_to("prop", pat);
    }
    pub fn add_slots(&mut self, pat: &Pat) {
        self.add_to_state(pat);
        self.add_to("slots", pat);
    }
    pub fn add_context(&mut self, pat: &Pat) {
        self.add_to_state(pat);
        self.add_to("context", pat);
    }
}

impl Default for IdentMap {
    fn default() -> Self {
        IdentMap {
            scope: IdentSet::new(),
            state: IdentSet::new(),
            array_state: IdentSet::new(),
            async_state: IdentSet::new(),
            memo: IdentSet::new(),
            prop: IdentSet::new(),
            slots: IdentSet::new(),
            context: IdentSet::new(),
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
