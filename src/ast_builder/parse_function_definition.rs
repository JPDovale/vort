use crate::tokenizer::{Token, TokenType};

use super::{parse_block::parse_block, Node, NodeId, NodeIdType, NodeType};
use std::{iter::Peekable, slice::Iter};

pub fn parse_function_definition(childs: &mut Vec<Node>, tokens: &mut Peekable<Iter<Token>>) {
    if let Some(f_token) = tokens.next() {
        if let Some(s_token) = tokens.next() {
            if let TokenType::Identifier(name) = &s_token.token_type {
                let body = parse_block(tokens);

                let node = Node {
                    _type: NodeType::FunctionDefinition,
                    line: f_token.line,
                    start: f_token.start,
                    end: f_token.end,
                    init: None,
                    body: Some(body),
                    definitions: None,
                    id: Some(NodeId {
                        _type: NodeIdType::Identifier,
                        end: s_token.end,
                        start: s_token.start,
                        line: s_token.line,
                        name: name.to_string(),
                    }),
                };

                childs.push(node);
            } else {
                panic!("Expected function name definition");
            }
        } else {
            panic!("Expected function name definition");
        }
    } else {
        panic!("Expected function definition");
    }
}
