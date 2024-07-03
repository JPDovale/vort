use super::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub fn process_plus(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Token {
    let token = Token {
        token_type: TokenType::Plus,
        line: *line,
        start: *column,
        end: *column,
    };

    chars.next();
    *column += 1;

    return token;
}

pub fn process_eq(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Token {
    let token = Token {
        token_type: TokenType::Eq,
        line: *line,
        start: *column,
        end: *column,
    };

    chars.next();
    *column += 1;

    return token;
}
