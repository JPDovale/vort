mod parse_block;
mod parse_definition;
mod parse_file;
mod parse_function_definition;
mod parse_primitive;
mod parse_variable_definition;
mod parse_variable_initalization;

use crate::tokenizer::Token;
use parse_file::parse_file;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    File,
    Definition,
    Block,
    VariableDefinition,
    FunctionDefinition,
}

#[derive(Debug, PartialEq)]
pub enum NodeIdType {
    Identifier,
}

#[derive(Debug, PartialEq)]
pub enum NodePrimitiveType {
    Int,
    String,
}

#[derive(Debug, PartialEq)]
pub enum NodeInitType {
    Int,
    String,
    Expression,
}

#[derive(Debug, PartialEq)]
pub enum NodeExpressionOperator {
    Add,
}

#[derive(Debug, PartialEq)]
pub struct NodePrimitive {
    pub _type: NodePrimitiveType,
    start: usize,
    end: usize,
    line: usize,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct NodeExpression {
    pub left: Box<NodeInit>,
    pub operator: NodeExpressionOperator,
    pub right: Box<NodeInit>,
}

#[derive(Debug, PartialEq)]
pub struct NodeInit {
    pub _type: NodeInitType,
    start: usize,
    end: usize,
    line: usize,
    pub value: Option<NodePrimitive>,
    pub expression: Option<NodeExpression>,
}

#[derive(Debug, PartialEq)]
pub struct NodeId {
    pub _type: NodeIdType,
    start: usize,
    end: usize,
    line: usize,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct NodeBody {
    pub _type: NodeType,
    start: usize,
    end: usize,
    line: usize,
    pub items: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub _type: NodeType,
    start: usize,
    end: usize,
    line: usize,
    pub id: Option<NodeId>,
    pub init: Option<NodeInit>,
    pub definitions: Option<Vec<Node>>,
    pub body: Option<NodeBody>,
}

#[derive(Debug, PartialEq)]
pub struct NodeFile {
    pub _type: NodeType,
    start: usize,
    end: usize,
    line: usize,
    pub items: Vec<Node>,
}

pub fn build_ast(tokens: &Vec<Token>) -> Vec<NodeFile> {
    let mut tokens_iter = tokens.iter().peekable();
    let mut ast = Vec::new();

    while let Some(_) = tokens_iter.peek() {
        ast.push(parse_file(&mut tokens_iter));
    }

    return ast;
}
