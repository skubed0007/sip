use smallstr::SmallString;

/// Inline capacity for identifiers (bytes).
pub const ID_CAP: usize = 64;

/// AST node with minimal allocations.
#[derive(Debug, Clone)]
pub enum NT {
    /// External declaration:
    /// - `name`: SIP identifier (inline or heap)
    /// - `params`: input variables
    /// - `c_name`: C function name (inline or heap)
    /// - `args`: arguments for the C call
    Extern(
        SmallString<[u8; ID_CAP]>,
        Vec<Var>,
        SmallString<[u8; ID_CAP]>,
        Vec<Var>,
    ),
    /// Link to one or more C libraries by name.
    /// - `names` as `Vec<SmallString<[u8; ID_CAP]>>`
    Link(Vec<SmallString<[u8; ID_CAP]>>),
    /// Function call:
    /// - `name`: SIP identifier (inline or heap)
    /// - `args`: arguments for the SIP function
    fncall(
        SmallString<[u8; ID_CAP]>,
        Vec<Var>,
    ),
}

/// Typed value with inline‑optimized name.
#[derive(Debug, Clone)]
pub enum Var {
    /// 32‑bit float + name
    F32(f32, SmallString<[u8; ID_CAP]>),
    /// 32‑bit signed int + name
    I32(i32, SmallString<[u8; ID_CAP]>),
    List(Vec<Var>),
    /// Variadic placeholder (zero‑sized)
    #[allow(clippy::exhaustive_enums)]
    Variadic,
    ///Custom Placeholder Types
    Generic(SmallString<[u8; ID_CAP]>),
}
