// Copyright 2020 the The Alef Component authors. All rights reserved. MIT license.

use super::css::CSS;
use super::jsx::JSX;
use swc_ecma_ast::*;

pub struct VarStatement {
  pub name: Pat,
  pub expr: Option<Box<Expr>>,
  is_ref: bool, // match typed `Ref<T>`
}

pub enum ConstKind {
  Const,   // match regular `const`
  Memo,    // match computed(memo) `const`
  Prop,    // match typed `Prop<T>`
  Slots,   // match typed `Prop<Children>`
  Context, // match typed `Context<T>`
}

pub struct ConstStatement {
  pub kind: ConstKind,
  pub name: Pat,
  pub expr: Box<Expr>,
}

pub struct EffectStatement {
  pub name: Option<String>, // the named effect is like `$_{NAME}:`
  pub expr: Box<Expr>,
}

pub struct JSXStatement {
  pub expr: Box<JSX>,
}

pub struct StyleStatement {
  pub css: Box<CSS>,
}

pub enum Statement {
  Var(VarStatement),       // match `var` and `let`
  Const(ConstStatement),   // match `const`
  Effect(EffectStatement), // match `$:` and `$_{NAME}:`
  JSX(JSXStatement),       // match `$t:`
  Style(StyleStatement),   // match `$style:`
}

/// AST for Alef Component.
pub struct AST {
  pub statements: Vec<Statement>,
}
