use crate::{err::defs::ErrT, parser::defs::{NodeT, Var}, token::defs::Token};

pub fn retp<'a>(
    _tokiter: &'a mut std::iter::Peekable<std::slice::Iter<'a, Token>>,
    _errlist: &'a mut Vec<ErrT>,
) -> NodeT<'a> {
    NodeT::Ret(Var{
        name : String::from("_TEST_"),
        var_type : crate::parser::defs::VarType::Char(' '),
        size : None
    })
}
