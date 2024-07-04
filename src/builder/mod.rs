use crate::ast_builder::{Node, NodeExpressionOperator, NodeFile, NodeInit, NodeType};
use std::fmt::Write;

pub fn generate_rust_code(ast: &Vec<NodeFile>) -> String {
    let mut rust_code = String::new();

    for node in ast {
        match node._type {
            NodeType::File => {
                rust_code.push_str(&nodes_to_rust(&node.items));
            }
            _ => panic!("Not implemented"),
        }
    }

    return rust_code;
}

fn nodes_to_rust(nodes: &Vec<Node>) -> String {
    let mut rust_code = String::new();

    for child in nodes {
        writeln!(rust_code, "{}", &node_to_rust(&child))
            .unwrap_or_else(|_| panic!("Error writing code"));
    }

    return rust_code;
}

fn node_init_to_rust(node: &NodeInit) -> String {
    let mut rust_code = String::new();

    if let Some(node_value) = &node.value {
        rust_code.push_str(&node_value.value);
        return rust_code;
    }

    if let Some(expression) = &node.expression {
        rust_code.push_str(&node_init_to_rust(&expression.left));

        match expression.operator {
            NodeExpressionOperator::Add => rust_code.push_str(" + "),
        }

        rust_code.push_str(&node_init_to_rust(&expression.right));

        return rust_code;
    }

    panic!("Not implemented")
}

fn node_to_rust(node: &Node) -> String {
    match node._type {
        NodeType::Definition => {
            if let Some(definitions) = &node.definitions {
                let mut rust_code = String::new();

                for definition in definitions {
                    rust_code.push_str(&node_to_rust(&definition));
                }

                return rust_code;
            } else {
                panic!("Expected definitions")
            }
        }
        NodeType::VariableDefinition => {
            if let Some(node_id) = &node.id {
                if let Some(node_init) = &node.init {
                    let mut rust_code = String::new();

                    rust_code.push_str("let mut ");
                    rust_code.push_str(&node_id.name);
                    rust_code.push_str(" = ");
                    rust_code.push_str(&node_init_to_rust(&node_init));
                    rust_code.push_str(";\n");

                    return rust_code;
                } else {
                    panic!("Expected variable init")
                }
            } else {
                panic!("Expected node id")
            }
        }
        NodeType::FunctionDefinition => {
            if let Some(node_id) = &node.id {
                if let Some(body) = &node.body {
                    let mut rust_code = String::new();

                    rust_code.push_str("fn ");

                    if node_id.name.eq("str") {
                        rust_code.push_str("main");
                    } else {
                        rust_code.push_str(&node_id.name);
                    }

                    rust_code.push_str("() {\n");
                    rust_code.push_str(&nodes_to_rust(&body.items));
                    rust_code.push_str("}");

                    return rust_code;
                } else {
                    panic!("Expected function body")
                }
            } else {
                panic!("Expected node id")
            }
        }
        _ => panic!("Not implemented"),
    }
}
