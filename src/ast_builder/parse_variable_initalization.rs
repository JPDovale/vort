use super::{parse_primitive::parse_primitive, NodeExpressionOperator, NodeInit, NodeInitType};
use crate::{
    ast_builder::NodeExpression,
    tokenizer::{Token, TokenType},
};
use std::{iter::Peekable, slice::Iter};

pub fn parse_variable_initialization(tokens: &mut Peekable<Iter<Token>>) -> NodeInit {
    if let Some(token) = tokens.next() {
        match token.token_type {
            TokenType::Number(_) => {
                if let Some(f_token) = tokens.next() {
                    let value = parse_primitive(token);

                    let left = NodeInit {
                        _type: NodeInitType::Int,
                        line: token.line,
                        start: token.start,
                        end: token.end,
                        value: Some(value),
                        expression: None,
                    };

                    match f_token.token_type {
                        TokenType::Semicolon => return left,
                        TokenType::Plus => {
                            let right = parse_variable_initialization(tokens);

                            let expression = NodeExpression {
                                operator: NodeExpressionOperator::Add,
                                left: Box::new(left),
                                right: Box::new(right),
                            };

                            return NodeInit {
                                _type: NodeInitType::Expression,
                                value: None,
                                line: token.line,
                                start: token.start,
                                end: token.end,
                                expression: Some(expression),
                            };
                        }
                        _ => panic!("Unexpected end of tokens"),
                    }
                } else {
                    panic!("Unexpected end of tokens")
                }
            }
            TokenType::Identifier(_) => {
                if let Some(f_token) = tokens.next() {
                    let value = parse_primitive(token);

                    let left = NodeInit {
                        _type: NodeInitType::String,
                        line: token.line,
                        start: token.start,
                        end: token.end,
                        value: Some(value),
                        expression: None,
                    };

                    match f_token.token_type {
                        TokenType::Semicolon => return left,
                        TokenType::Plus => {
                            let right = parse_variable_initialization(tokens);

                            let expression = NodeExpression {
                                operator: NodeExpressionOperator::Add,
                                left: Box::new(left),
                                right: Box::new(right),
                            };

                            return NodeInit {
                                _type: NodeInitType::Expression,
                                value: None,
                                line: token.line,
                                start: token.start,
                                end: token.end,
                                expression: Some(expression),
                            };
                        }
                        _ => panic!("Unexpected end of tokens"),
                    }
                } else {
                    panic!("Unexpected end of tokens")
                }
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    } else {
        panic!("Unexpected end of tokens")
    }
}
