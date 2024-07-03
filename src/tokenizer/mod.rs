mod delimiters;
mod identifier;
mod number;
mod operators;

use delimiters::{process_comma, process_lbracket, process_rbracket, process_semicolon};
use identifier::process_identifier;
use number::process_number;
use operators::{process_eq, process_plus};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Number(i64),
    Plus,
    Eq,
    // Minus,
    // Asterisk,
    // Slash,
    // LParen,
    // RParen,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Fn,
    Def,
    Var,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(&char) = chars.peek() {
        match char {
            '0'..='9' => tokens.push(process_number(&mut chars, &mut line, &mut column)),
            'a'..='z' => tokens.push(process_identifier(&mut chars, &mut line, &mut column)),
            'A'..='Z' => tokens.push(process_identifier(&mut chars, &mut line, &mut column)),
            '_' => tokens.push(process_identifier(&mut chars, &mut line, &mut column)),
            '{' => tokens.push(process_lbracket(&mut chars, &mut line, &mut column)),
            '}' => tokens.push(process_rbracket(&mut chars, &mut line, &mut column)),
            ';' => tokens.push(process_semicolon(&mut chars, &mut line, &mut column)),
            ',' => tokens.push(process_comma(&mut chars, &mut line, &mut column)),
            '+' => tokens.push(process_plus(&mut chars, &mut line, &mut column)),
            '=' => tokens.push(process_eq(&mut chars, &mut line, &mut column)),
            '"' => tokens.push(process_identifier(&mut chars, &mut line, &mut column)),
            ' ' | '\t' => {
                chars.next();
                column += 1;
            }
            '\r' => {
                chars.next();
            }
            '\n' => {
                chars.next();
                line += 1;
                column = 1;
            }
            _ => return Err(format!("Unexpected character: {}", char)),
        }
    }

    Ok(tokens)
}
