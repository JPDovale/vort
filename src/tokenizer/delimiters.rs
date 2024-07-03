use super::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub fn process_comma(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Token {
    let token = Token {
        token_type: TokenType::Comma,
        line: *line,
        start: *column,
        end: *column,
    };

    chars.next();
    *column += 1;

    return token;
}

pub fn process_semicolon(
    chars: &mut Peekable<Chars>,
    line: &mut usize,
    column: &mut usize,
) -> Token {
    let token = Token {
        token_type: TokenType::Semicolon,
        line: *line,
        start: *column,
        end: *column,
    };

    chars.next();
    *column += 1;

    return token;
}

pub fn process_lbracket(
    chars: &mut Peekable<Chars>,
    line: &mut usize,
    column: &mut usize,
) -> Token {
    let token = Token {
        token_type: TokenType::LBracket,
        line: *line,
        start: *column,
        end: *column,
    };

    chars.next();
    *column += 1;

    return token;
}

pub fn process_rbracket(
    chars: &mut Peekable<Chars>,
    line: &mut usize,
    column: &mut usize,
) -> Token {
    let token = Token {
        token_type: TokenType::RBracket,
        line: *line,
        start: *column,
        end: *column,
    };

    chars.next();
    *column += 1;

    return token;
}
