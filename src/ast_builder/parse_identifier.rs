use crate::tokenizer::{Token, TokenType};

use super::{Node, NodeId, NodeIdType, NodeType};
use std::{iter::Peekable, slice::Iter};

pub fn parse_identifier(childs: &mut Vec<Node>, tokens: &mut Peekable<Iter<Token>>) {
    if let Some(f_token) = tokens.next() {
        if let TokenType::Identifier(name) = &f_token.token_type {
            if let Some(s_token) = tokens.next() {
                match &s_token.token_type {
                    TokenType::LParen => {
                        tokens.next();
                        let node = Node {
                            _type: NodeType::CallExpression,
                            definitions: None,
                            id: None,
                            end: s_token.end,
                            start: f_token.start,
                            line: f_token.line,
                            init: None,
                            body: None,
                            callee: Some(NodeId {
                                line: s_token.line,
                                start: f_token.start,
                                end: s_token.end,
                                _type: NodeIdType::Identifier,
                                name: name.clone(),
                            }),
                        };

                        tokens.next();
                        childs.push(node);
                    }
                    _ => panic!("no paren"),
                }
            }
        } else {
            panic!("no identifier")
        }
    } else {
        panic!("no identifier")
    }
}
