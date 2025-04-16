use crate::token::defs::TT;

use super::Parser;

impl Parser {
    pub fn parse(&mut self) {
        match self.cur.TT {
            TT::FN => {
                self.mv();
                self.pfunc();
            }
            _ => {
                eprintln!("not defined");
            }
        }
    }
}