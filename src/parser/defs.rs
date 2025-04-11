#[derive(Debug)]
/// Represents different kinds of syntax tree nodes in the Sip language.
///
/// These nodes form the AST and represent distinct syntactic elements.
pub enum NodeT<'a> {
    /// Variable definition.
    ///
    /// Syntax: `var <name>: <type> [size]`
    VarDef(Var),

    /// Function definition.
    ///
    /// Syntax: `fun <name>(<args>) @ <ret_type> { <body> }`
    FunDef(FunDef<'a>),
}
#[derive(Debug)]
/// Defines a function signature and body in the Sip language.
///
/// This structure includes the function name, arguments, optional return type,
/// and the full body represented as a list of AST nodes.
pub struct FunDef<'a> {
    /// Name of the function.
    pub name: &'a str,

    /// Optional list of arguments.
    ///
    /// Each argument is represented by a `Var` structure.
    pub args: Option<Vec<Var>>,

    /// List of AST nodes forming the body of the function.
    pub body: Vec<NodeT<'a>>,

    /// Optional return type of the function.
    pub ret_type: Option<VarType>,
}
#[derive(Debug)]
/// Represents a variable in the Sip language.
///
/// A variable has a name, a type, and optionally a size (only for special types).
pub struct Var {
    /// Name of the variable.
    pub name: String,

    /// Type of the variable.
    pub var_type: VarType,

    /// Optional size, only used for types like `LPage` and `RPage`.
    pub size: Option<usize>,
}
/// Enumerates the supported types in Sip.
///
/// These types are used in variable declarations, return types, etc.
#[derive(Clone, Debug,PartialEq, Eq)]
pub enum VarType {
    /// 32-bit signed integer
    I32,

    /// 64-bit signed integer
    I64,

    /// 32-bit floating point
    F32,

    /// 64-bit floating point
    F64,

    /// Character (usually UTF-8 or ASCII)
    Char,

    /// Left Page — used for memory/block access (with `size`)
    LazyPage,

    /// Right Page — used for memory/block access (with `size`)
    ZeroPage,
    /// Multiple types grouped in a tuple-like structure
    Tuple(Vec<VarType>),
    ///Nil type
    Nil,
}
