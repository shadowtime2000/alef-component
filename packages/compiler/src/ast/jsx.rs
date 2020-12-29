// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::{statement::*, transformer::StatementsTransformer};
use crate::resolve::{format_component_name, Resolver};
use indexmap::{IndexMap, IndexSet};
use regex::Regex;
use std::{cell::RefCell, path::Path, rc::Rc};
use swc_common::{iter::IdentifyLast, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_utils::{quote_ident, ExprFactory, HANDLER};
use swc_ecma_visit::{noop_fold_type, Fold};

impl StatementsTransformer {
    pub fn transform2(&self) {
        
    }
}

fn jsx_name(name: JSXElementName) -> Box<Expr> {
    match name {
        JSXElementName::Ident(i) => {
            if i.sym.eq("this") {
                Box::new(Expr::This(ThisExpr { span: DUMMY_SP }))
            } else if i.sym.chars().next().unwrap().is_ascii_lowercase() {
                Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: i.sym,
                    has_escape: false,
                })))
            } else {
                Box::new(Expr::Ident(i))
            }
        }
        JSXElementName::JSXNamespacedName(JSXNamespacedName { name, .. }) => {
            HANDLER.with(|handler| {
                handler
                    .struct_span_err(
                        name.span,
                        "Alep Component does not support JSX Namespace yet.",
                    )
                    .emit()
            });
            Box::new(Expr::Invalid(Invalid { span: DUMMY_SP }))
        }
        JSXElementName::JSXMemberExpr(JSXMemberExpr { obj, prop }) => {
            Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: convert_jsx_obj(obj),
                prop: Box::new(Expr::Ident(prop)),
                computed: false,
            }))
        }
    }
}

fn convert_jsx_obj(obj: JSXObject) -> ExprOrSuper {
    match obj {
        JSXObject::Ident(i) => {
            if i.sym.eq("this") {
                return ExprOrSuper::Expr(Box::new(Expr::This(ThisExpr { span: DUMMY_SP })));
            }
            i.as_obj()
        }
        JSXObject::JSXMemberExpr(e) => {
            let e = *e;
            MemberExpr {
                span: DUMMY_SP,
                obj: convert_jsx_obj(e.obj),
                prop: Box::new(Expr::Ident(e.prop)),
                computed: false,
            }
            .as_obj()
        }
    }
}

fn attr_to_prop(a: JSXAttr) -> Prop {
    let key = to_prop_name(a.name);
    let value = a
        .value
        .map(|v| match v {
            JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::Expr(e),
                ..
            }) => e,
            JSXAttrValue::JSXElement(e) => Box::new(Expr::JSXElement(e)),
            JSXAttrValue::JSXFragment(e) => Box::new(Expr::JSXFragment(e)),
            JSXAttrValue::Lit(lit) => Box::new(lit.into()),
            JSXAttrValue::JSXExprContainer(JSXExprContainer {
                expr: JSXExpr::JSXEmptyExpr(_),
                ..
            }) => unreachable!("attr_to_prop(JSXEmptyExpr)"),
        })
        .unwrap_or_else(|| {
            Box::new(Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: true,
            })))
        });
    Prop::KeyValue(KeyValueProp { key, value })
}

fn to_prop_name(n: JSXAttrName) -> PropName {
    match n {
        JSXAttrName::Ident(i) => {
            if i.sym.contains('-') {
                PropName::Str(Str {
                    span: DUMMY_SP,
                    value: i.sym,
                    has_escape: false,
                })
            } else {
                PropName::Ident(i)
            }
        }
        JSXAttrName::JSXNamespacedName(JSXNamespacedName { ns, name }) => PropName::Str(Str {
            span: DUMMY_SP,
            value: format!("{}:{}", ns.sym, name.sym).into(),
            has_escape: false,
        }),
    }
}

lazy_static! {
    static ref SPACE_NL_START: Regex = Regex::new("^\\s*\n\\s*").unwrap();
    static ref SPACE_NL_END: Regex = Regex::new("\\s*\n\\s*$").unwrap();
}

fn jsx_text_to_string(t: &str) -> String {
    if t.eq(" ") {
        return t.into();
    }
    if !t.contains(' ') && !t.contains('\n') {
        return t.into();
    }

    let s = SPACE_NL_START.replace_all(&t, "");
    let s = SPACE_NL_END.replace_all(&s, "");
    let need_leading_space = s.starts_with(' ');
    let need_trailing_space = s.ends_with(' ');

    let mut buf = String::new();

    if need_leading_space {
        buf.push(' ')
    }

    for (last, s) in s.split_ascii_whitespace().identify_last() {
        buf.push_str(s);
        if !last {
            buf.push(' ');
        }
    }

    if need_trailing_space && !buf.ends_with(' ') {
        buf.push(' ');
    }

    buf.into()
}
