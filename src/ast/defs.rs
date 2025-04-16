#[derive(Debug)]
pub enum NodeT {
    FUNCNode(FUNC),
}
#[derive(Debug)]
pub enum VarT {
    I32(i32),
    U8(u8),
    I64(i64),
    F32(f32),
    F64(f64),
}
#[derive(Debug)]
#[allow(non_snake_case, unused)]
pub struct Var {
    pub name: String,
    pub dyna: bool,
    pub TT: VarT,
}
#[derive(Debug)]
#[allow(non_snake_case, unused)]
pub struct FUNC {
    pub name: String,
    pub args: Option<Vec<Var>>,
    pub body: Vec<NodeT>,
    pub ret: Var,
}
