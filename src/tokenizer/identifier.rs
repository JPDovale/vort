use super::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub fn process_identifier(
    chars: &mut Peekable<Chars>,
    line: &mut usize,
    column: &mut usize,
) -> Token {
    let start_column = *column;
    let mut identifier = String::new();

    while let Some(&c) = chars.peek() {
        if !c.is_alphanumeric() && c != '_' && c != '"' {
            break;
        };

        identifier.push(c);
        chars.next();
        *column += 1;
    }

    let token_type = match identifier.as_str() {
        "fn" => TokenType::Fn,
        "def" => TokenType::Def,
        "var" => TokenType::Var,
        _ => TokenType::Identifier(identifier),
    };

    return Token {
        token_type,
        line: *line,
        start: start_column,
        end: *column - 1,
    };
}
