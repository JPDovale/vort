use super::{parse_definition::parse_definition, NodeBody, NodeType};
use crate::tokenizer::{Token, TokenType};
use std::{iter::Peekable, slice::Iter};

pub fn parse_block(tokens: &mut Peekable<Iter<Token>>) -> NodeBody {
    tokens.next();
    let mut childs = Vec::new();

    while let Some(token) = tokens.peek() {
        match token.token_type {
            TokenType::Def => parse_definition(&mut childs, tokens),
            _ => {
                tokens.next();
            }
        }
    }

    let node = NodeBody {
        line: 0,
        start: 0,
        end: 0,
        _type: NodeType::Block,
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
