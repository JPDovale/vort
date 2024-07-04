use crate::tokenizer::{Token, TokenType};
use std::{iter::Peekable, slice::Iter};

use super::{
    parse_function_definition::parse_function_definition,
    parse_variable_definition::parse_variable_definition, Node, NodeType,
};

pub fn parse_definition(childs: &mut Vec<Node>, tokens: &mut Peekable<Iter<Token>>) {
    tokens.next();
    let mut definitions = Vec::new();

    while let Some(token) = tokens.peek() {
        match token.token_type {
            TokenType::Var => parse_variable_definition(&mut definitions, tokens),
            TokenType::Fn => parse_function_definition(&mut definitions, tokens),
            _ => panic!("Invalid definition by parse_definition {:?}", token),
        }
    }

    let node = Node {
        _type: NodeType::Definition,
        definitions: Some(definitions),
        start: 0,
        end: 0,
        line: 0,
        id: None,
        init: None,
        body: None,
    };

    childs.push(node);

    // match tokens.peek() {
    //     Some(token) if token.token_type == TokenType::Var => {
    //         tokens.next();
    //         return parse_variable_definition(tokens);
    //     }
    //     _ => parse_expression(tokens),
    // }
    // TODO
}
