use super::{parse_definition::parse_definition, NodeFile, NodeType};
use crate::tokenizer::{Token, TokenType};
use std::{iter::Peekable, slice::Iter};

pub fn parse_file(tokens: &mut Peekable<Iter<Token>>) -> NodeFile {
    let mut childs = Vec::new();

    while let Some(token) = tokens.peek() {
        match token.token_type {
            TokenType::Def => parse_definition(&mut childs, tokens),
            _ => panic!("Invalid token"),
        }
    }

    let node = NodeFile {
        _type: NodeType::File,
        end: 0,
        line: 0,
        start: 0,
        items: childs,
    };

    // match tokens.peek() {
    //     Some(token) if token.token_type == TokenType::Def => {
    //         tokens.next();
    //         return parse_definition(tokens);
    //     }
    //     _ => tokens.next(),
    // }

    return node;
}
