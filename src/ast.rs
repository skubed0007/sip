use smallstr::SmallString;

/// Inline capacity for identifiers (bytes).
pub const ID_CAP: usize = 64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CLINKT {
    PATH,
    LIB,
}

/// AST node with minimal allocations.
#[derive(Debug, Clone)]
pub enum NT {
    /// External declaration
    Extern(
        SmallString<[u8; ID_CAP]>,
        Vec<Var>,
        SmallString<[u8; ID_CAP]>,
        Vec<Var>,
    ),
    /// Link to one or more C libraries or paths
    /// Each item is (name, type)
    Link(Vec<(SmallString<[u8; ID_CAP]>, CLINKT)>),
    /// Function call
    FNCALL(SmallString<[u8; ID_CAP]>, Vec<Var>),
}

/// Typed value with inline-optimized name
#[derive(Debug, Clone)]
pub enum Var {
    F32(f32, SmallString<[u8; ID_CAP]>),
    I32(i32, SmallString<[u8; ID_CAP]>),
    List(Vec<Var>),
    Variadic,
    Generic(SmallString<[u8; ID_CAP]>),
}
