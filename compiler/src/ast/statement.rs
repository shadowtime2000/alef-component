use super::{css::CSS, identmap::IdentMap};
use swc_ecma_ast::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportStatement {
    pub specifiers: Vec<ImportSpecifier>,
    pub src: String,
    pub is_alef_component: bool, // match import App from "./*.alef"
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VarStatement {
    pub name: Pat,
    pub init: Option<Expr>,
    pub is_ref: bool,   // match typed `Ref<T>`
    pub is_array: bool, // match typed `Array<T>`
    pub is_async: bool, // match `let data = await ...`
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstTyped {
    Regular, // match regular `const`
    Memo,    // match typed `Memo<T>`
    Prop,    // match typed `Prop<T>`
    Slots,   // match typed `Prop<Slots>`
    Context, // match typed `Context<N, T=any>`
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstStatement {
    pub name: Pat,
    pub typed: ConstTyped,
    pub init: Expr,
    pub ctx_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FCStatement {
    pub scope_idents: IdentMap,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SideEffectStatement {
    pub name: Option<String>, // a named side effect is like `$_{NAME}:`
    pub stmt: Stmt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TemplateStatement {
    Element(JSXElement),
    Fragment(JSXFragment),
    If(IfStmt),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyleStatement {
    pub css: CSS,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportStatement {
    pub expr: Expr,
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
