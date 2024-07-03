use super::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub fn process_number(chars: &mut Peekable<Chars>, line: &mut usize, column: &mut usize) -> Token {
    let start_column = *column;
    let mut number = 0 as i64;

    while let Some(&digit) = chars.peek() {
        if !digit.is_digit(10) {
            break;
        }

        number = number * 10 + digit.to_digit(10).unwrap() as i64;
        chars.next();
        *column += 1;
    }

    return Token {
        token_type: TokenType::Number(number),
        line: *line,
        start: start_column,
        end: *column - 1,
    };
}
