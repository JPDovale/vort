mod tokenizer;

use std::fmt::Write;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter::Peekable,
    slice::Iter,
};
use tokenizer::{tokenize, Token, TokenType};

#[derive(Debug, PartialEq)]
enum NodeType {
    Program,
    Definition,
    Primary,
    // VariableDefinition,
}

#[derive(Debug, PartialEq)]
enum NodeIdTye {
    Identifier,
    Numric,
    String,
}

#[derive(Debug, PartialEq)]
struct NodeId {
    node_type: NodeIdTye,
    start: usize,
    end: usize,
    line: usize,
    name: Option<String>,
    value: Option<String>,
}

#[derive(Debug, PartialEq)]
struct Node {
    node_type: NodeType,
    id: Option<NodeId>,
    start: usize,
    end: usize,
    line: usize,
    init: Option<NodeId>,
    body: Option<Vec<Node>>,
    definitions: Option<Vec<Node>>,
}

fn build_ast(tokens: &Vec<Token>) -> Vec<Result<Node, String>> {
    let mut tokens_iter = tokens.iter().peekable();
    let mut ast = Vec::new();

    while let Some(_) = tokens_iter.peek() {
        ast.push(parse_statement(&mut tokens_iter));
    }

    return ast;
}

fn parse_statement(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, String> {
    match tokens.peek() {
        Some(token) if token.token_type == TokenType::Def => {
            tokens.next();
            return parse_definition(tokens);
        }
        _ => parse_expression(tokens),
    }
}

fn parse_definition(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, String> {
    match tokens.peek() {
        Some(token) if token.token_type == TokenType::Var => {
            tokens.next();
            return parse_variable_definition(tokens);
        }
        _ => parse_expression(tokens),
    }
    // TODO
}

fn parse_variable_definition(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, String> {
    if let Some(token) = tokens.next() {
        if let TokenType::Identifier(name) = &token.token_type {
            if let Some(token) = tokens.next() {
                if token.token_type == TokenType::Eq {
                    let value = parse_expression(tokens)?;

                    if let Some(token) = tokens.next() {
                        if token.token_type == TokenType::Semicolon {
                            return Ok(Node {
                                start: token.start,
                                end: token.end,
                                line: token.line,
                                init: None,
                                definitions: Some(vec![value]),
                                node_type: NodeType::Definition,
                                id: Some(NodeId {
                                    node_type: NodeIdTye::Identifier,
                                    start: token.start,
                                    end: token.end,
                                    line: token.line,
                                    value: None,
                                    name: Some(name.to_string()),
                                }),
                                body: None,
                            });
                        } else {
                            return Err(format!(
                                "Expected ';' after variable definition, found {:?}",
                                token
                            ));
                        }
                    } else {
                        return Err("Expected expression after '='".to_string());
                    }
                } else {
                    return Err(format!(
                        "Expected '=' after variable name, found {:?}",
                        token
                    ));
                }
            } else {
                return Err("Expected expression after '='".to_string());
            }
        } else {
            return Err("Expected variable name".to_string());
        }
    } else {
        return Err("Expected variable name".to_string());
    }
    // TODO
}

fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, String> {
    let left = parse_primary(tokens)?;

    // while let Some(token) = tokens.peek() {
    //     match token.token_type {
    //         TokenType::Quote => {
    //             tokens.next();
    //         }
    //         _ => break,
    //     }
    // }

    Ok(left)
    // TODO
}

fn parse_primary(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, String> {
    println!("parse_primary {:?}", tokens);
    if let Some(token) = tokens.next() {
        match &token.token_type {
            TokenType::Number(num) => Ok(Node {
                start: token.start,
                end: token.end,
                line: token.line,
                node_type: NodeType::Primary,
                id: None,
                body: None,
                init: Some(NodeId {
                    line: token.line,
                    start: token.start,
                    end: token.end,
                    name: None,
                    node_type: NodeIdTye::Numric,
                    value: Some(num.to_string()),
                }),
                definitions: None,
            }),
            TokenType::Identifier(ident) => Ok(Node {
                start: token.start,
                end: token.end,
                line: token.line,
                node_type: NodeType::Primary,
                id: None,
                body: None,
                init: Some(NodeId {
                    line: token.line,
                    start: token.start,
                    end: token.end,
                    name: None,
                    node_type: NodeIdTye::String,
                    value: Some(ident.to_string()),
                }),
                definitions: None,
            }),
            _ => Err(format!("Unexpected token: {:?}", token)),
        }
    } else {
        Err("Expected primary expression".to_string())
    }
}

fn generate_rust_code(ast: &Vec<Result<Node, String>>) -> String {
    let mut rust_code = String::new();

    for node_raw in ast {
        let node = node_raw.as_ref().unwrap();
        match node.node_type {
            NodeType::Definition => {
                if let Some(node_id) = &node.id {
                    if let Some(name) = &node_id.name {
                        writeln!(
                            &mut rust_code,
                            "let mut {} = {};",
                            name,
                            node_to_rust(&node).unwrap()
                        )
                        .unwrap_or_else(|_| ());
                    }
                }
            }
            _ => {
                writeln!(&mut rust_code, "{}", node_to_rust(&node).unwrap()).unwrap_or_else(|_| ())
            }
        }
    }

    return rust_code;
}

fn node_to_rust(node: &Node) -> Result<String, String> {
    match node.node_type {
        NodeType::Program => Ok("".to_string()),
        NodeType::Definition => {
            if let Some(node_definitions) = &node.definitions {
                let mut rust_code = String::new();

                for node_definition in node_definitions {
                    rust_code.push_str(&node_to_rust(node_definition)?);
                }

                return Ok(rust_code.to_string());
            } else {
                panic!("Expected definitions")
            }
        }
        NodeType::Primary => {
            if let Some(node_init) = &node.init {
                match node_init.node_type {
                    NodeIdTye::Numric => {
                        if let Some(value) = &node_init.value {
                            return Ok(value.to_string());
                        }

                        Err("Expected value".to_string())
                    }
                    NodeIdTye::String => {
                        if let Some(value) = &node_init.value {
                            return Ok(format!("{}", value));
                        }

                        Err("Expected value".to_string())
                    }
                    _ => Err("Node type not implemented".to_string()),
                }
            } else {
                Err("Expected id".to_string())
            }
        }
        _ => Err("Node type not implemented".to_string()),
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    let mut content = String::new();

    for line in BufReader::new(file).lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "test/test.vt";
    let input = read_file(filename)?;

    let tokens = tokenize(&input)?;
    let ast = build_ast(&tokens);

    dbg!("{:?}", &ast);

    let rust_code = generate_rust_code(&ast);

    println!("{}", rust_code);

    Ok(())
}
