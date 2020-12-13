mod css;
mod jsx;
mod transform;
mod walker;

use crate::resolve::Resolver;
use css::CSS;
use jsx::JSX;
use std::{cell::RefCell, rc::Rc};
use swc_ecma_ast::*;
use swc_ecma_visit::Fold;
use transform::ASTransform;
use walker::ASTWalker;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportStatement {
  pub specifiers: Vec<ImportSpecifier>,
  pub src: String,
  is_alef_component: bool, // match import App from "./App.alef"
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VarStatement {
  pub name: Pat,
  pub expr: Option<Box<Expr>>,
  is_array: bool, // match typed `Array<T>`
  is_ref: bool,   // match typed `Ref<T>`
  is_async: bool, // match `let data = await fetch(...)`
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstKind {
  Const,   // match regular `const`
  Memo,    // match computed(memo) `const`
  Prop,    // match typed `Prop<T>`
  Slots,   // match typed `Prop<Children>`
  Context, // match typed `Context<T>`
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstStatement {
  pub kind: ConstKind,
  pub name: Pat,
  pub expr: Box<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SideEffectStatement {
  pub name: Option<String>, // the named side effect is like `$_{NAME}:`
  pub stmt: Box<Stmt>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JSXStatement {
  pub jsx: Box<JSX>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyleStatement {
  pub css: Box<CSS>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportStatement {
  pub specifiers: Vec<Ident>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
  Import(ImportStatement),         // match `import ... from "..."`
  Var(VarStatement),               // match `var` and `let`
  Const(ConstStatement),           // match `const`
  SideEffect(SideEffectStatement), // match `$:` and `$_{NAME}:`
  JSX(JSXStatement),               // match `$t:`
  Style(StyleStatement),           // match `$style:`
  Export(ExportStatement),         // match `export default { ... }`
  Stmt(Stmt),                      // regular statement
}

/// AST for Alef Component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AST {
  pub statements: Vec<Statement>,
}

pub fn ast_walker(resolver: Rc<RefCell<Resolver>>) -> impl Fold {
  ASTWalker {
    resolver: resolver.clone(),
  }
}

pub fn ast_trasnform(ast: &AST) -> Vec<Stmt> {
  ASTransform::from(ast).transform()
}
