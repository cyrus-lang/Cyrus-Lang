use core::fmt;

use crate::Lexer;

impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "pos: {}, next_pos: {}, char: {}", self.pos, self.next_pos, self.ch)
    }
}