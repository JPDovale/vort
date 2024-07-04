use crate::tokenizer::{Token, TokenType};

use super::{NodePrimitive, NodePrimitiveType};

pub fn parse_primitive(token: &Token) -> NodePrimitive {
    match &token.token_type {
        TokenType::Number(int) => {
            return NodePrimitive {
                _type: NodePrimitiveType::Int,
                line: token.line,
                end: token.end,
                start: token.start,
                value: int.to_string(),
            }
        }
        TokenType::Identifier(identifier) => {
            return NodePrimitive {
                _type: NodePrimitiveType::String,
                start: token.start,
                end: token.end,
                line: token.line,
                value: identifier.to_string(),
            };
        }
        _ => panic!("unexpected token: {:?}", token),
    }
}
