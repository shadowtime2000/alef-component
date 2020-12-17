use super::{css::CSS, identmap::IdentMap};
use swc_ecma_ast::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportStatement {
    pub specifiers: Vec<ImportSpecifier>,
    pub src: String,
    pub is_alef_component: bool, // match import App from "./App.alef"
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VarStatement {
    pub name: Pat,
    pub init: Option<Box<Expr>>,
    pub is_ref: bool,   // match typed `Ref<T>`
    pub is_array: bool, // match typed `Array<T>`
    pub is_async: bool, // match `let data = await fetch(...)`
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstTyped {
    Regular, // match regular `const`
    Memo,    // match typed `Memo<T>`
    Prop,    // match typed `Prop<T>`
    Slots,   // match typed `Prop<Slots>`
    Context, // match typed `Context<T>`
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstStatement {
    pub name: Pat,
    pub typed: ConstTyped,
    pub init: Box<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FCStatement {
    pub scope_idents: IdentMap,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SideEffectStatement {
    pub name: Option<String>, // a named side effect is like `$_{NAME}:`
    pub stmt: Box<Stmt>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TemplateStatement {
    Element(JSXElement),
    Fragment(JSXFragment),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyleStatement {
    pub css: Box<CSS>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportStatement {
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Import(ImportStatement),         // match `import ... from "..."`
    Var(VarStatement),               // match `var` and `let`
    Const(ConstStatement),           // match `const`
    FC(FCStatement),                 // match `FC<Props>`
    SideEffect(SideEffectStatement), // match `$:` and `$_{NAME}:`
    Template(TemplateStatement),     // match `$t:`
    Style(StyleStatement),           // match `$style:`
    Export(ExportStatement),         // match `export default { ... }`
    Stmt(Stmt),                      // regular statement
}
