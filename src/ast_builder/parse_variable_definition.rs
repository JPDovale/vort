use super::{
    parse_variable_initalization::parse_variable_initialization, Node, NodeId, NodeIdType, NodeType,
};
use crate::tokenizer::{Token, TokenType};
use std::{iter::Peekable, slice::Iter};

pub fn parse_variable_definition(childs: &mut Vec<Node>, tokens: &mut Peekable<Iter<Token>>) {
    tokens.next();

    if let Some(f_token) = tokens.next() {
        if let TokenType::Identifier(name) = &f_token.token_type {
            if let Some(s_token) = tokens.next() {
                if s_token.token_type == TokenType::Eq {
                    let init = parse_variable_initialization(tokens);

                    let node = Node {
                        _type: NodeType::VariableDefinition,
                        start: f_token.start,
                        end: s_token.end,
                        line: f_token.line,
                        definitions: None,
                        body: None,
                        callee: None,
                        init: Some(init),
                        id: Some(NodeId {
                            _type: NodeIdType::Identifier,
                            start: f_token.start,
                            end: f_token.end,
                            line: f_token.line,
                            name: name.to_string(),
                        }),
                    };

                    childs.push(node);
                    tokens.next();
                } else {
                    panic!("Expected '=' after variable name");
                }
            } else {
                panic!("Expected '=' after variable name");
            }
        } else {
            panic!("Expected variable name");
        }
    } else {
        panic!("Expected variable name");
    }
}
//
// pub fn parse_variable_definition(tokens: &mut Peekable<Iter<Token>>) -> Node {
//     if let Some(token) = tokens.next() {
//         if let TokenType::Identifier(name) = &token.token_type {
//             if let Some(token) = tokens.next() {
//                 if token.token_type == TokenType::Eq {
//                     let value = parse_expression(tokens)?;
//
//                     if let Some(token) = tokens.next() {
//                         if token.token_type == TokenType::Semicolon {
//                             return Node {
//                                 start: token.start,
//                                 end: token.end,
//                                 line: token.line,
//                                 init: None,
//                                 definitions: Some(vec![value]),
//                                 _type: NodeType::Definition,
//                                 id: Some(NodeId {
//                                     node_type: NodeIdTye::Identifier,
//                                     start: token.start,
//                                     end: token.end,
//                                     line: token.line,
//                                     value: None,
//                                     name: Some(name.to_string()),
//                                 }),
//                                 body: None,
//                             };
//                         } else {
//                             return Err(format!(
//                                 "Expected ';' after variable definition, found {:?}",
//                                 token
//                             ));
//                         }
//                     } else {
//                         return Err("Expected expression after '='".to_string());
//                     }
//                 } else {
//                     return Err(format!(
//                         "Expected '=' after variable name, found {:?}",
//                         token
//                     ));
//                 }
//             } else {
//                 return Err("Expected expression after '='".to_string());
//             }
//         } else {
//             return Err("Expected variable name".to_string());
//         }
//     } else {
//         return Err("Expected variable name".to_string());
//     }
//     // TODO
// }
