use std::collections::VecDeque;

use crate::lexer::{self, BinOpType, Token, TokenType, UnanyOpType};

type BNode = Box<Node>;

#[derive(Clone)]
enum Node {
    Identifier(String),
    Integer(i64),
    If(BNode, Vec<Node>, Vec<Node>),
    Function(BNode, Vec<Node>, Vec<Node>),
    Return(BNode),
    BinExp(BNode, BinOpType),
    UnExp(BNode, UnanyOpType),
    FunctionCall(BNode, Vec<Node>),
}

fn parser(tokens: &Vec<Token>) -> Vec<Node> {
    let mut nodes = vec![Node::Integer(10); 0];
    let mut cursor: usize = 0;

    while cursor < tokens.len() {
        match function(&tokens, &mut cursor) {
            Some(node) => nodes.push(node),
            None => error_handler(),
        }
    }
    nodes
}

fn function(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    if let TokenType::Fn = tokens[*cursor].t_type {
        *cursor += 1;
        Some(Node::Function(
            Box::new(identifier(tokens, cursor)?),
            argument(tokens, cursor)?,
            block_statement(tokens, cursor)?,
        ))
    } else {
        None
    }
}

fn block_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Vec<Node>> {
    let mut nodes = vec![Node::Integer(10); 0];
    match tokens[*cursor].t_type {
        TokenType::LeftCurl => {
            *cursor += 1;
            while match tokens[*cursor].t_type {
                TokenType::RightCurl => false,
                _ => true,
            } {
                match tokens[*cursor].t_type {
                    TokenType::If => {
                        *cursor += 1;
                        match tokens[*cursor].t_type {
                            TokenType::Identifier(_) => {
                                *cursor += 1;

                                let expression = match expression(tokens, cursor)? {
                                    Node::BinExp(node, op_type) => {
                                        *cursor += 1;
                                        Some(Box::new(Node::BinExp(node, op_type)))
                                    }

                                    Node::UnExp(node, op_type) => {
                                        *cursor += 1;

                                        Some(Box::new(Node::UnExp(node, op_type)))
                                    }

                                    _ => None,
                                };
                                nodes.push(Node::If(expression?, (), ()))
                            }
                            _ => panic!(),
                        }
                    }
                    TokenType::Return => {
                        *cursor += 1;
                        match tokens[*cursor].t_type {
                            TokenType::Identifier(_) => {*cursor+=1;
                                nodes.
                            },

                            _ => panic!(),
                        };
                        
                    }
                    _ => panic!(),
                }
            }
            *cursor += 1;
            Some(nodes)
        }
        _ => None,
    }
}

fn return_node(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {

}

fn expression(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {}
fn argument(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Vec<Node>> {
    let mut nodes = vec![Node::Integer(10); 0];
    match tokens[*cursor].t_type {
        TokenType::LeftPar => {
            while match tokens[*cursor].t_type {
                TokenType::Identifier(_) => true,
                _ => false,
            } {
                nodes.push(identifier(tokens, cursor)?)
            }
            *cursor += 1;
            Some(nodes)
        }
        _ => None,
    }
}

fn identifier(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    if let TokenType::Identifier(x) = &tokens[*cursor].t_type {
        *cursor += 1;
        Some(Node::Identifier(x.to_string()))
    } else {
        None
    }
}

fn integer(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    if let TokenType::Literal(x) = tokens[*cursor].t_type {
        *cursor += 1;
        Some(Node::Integer(x))
    } else {
        None
    }
}

fn error_handler() {
    panic!()
}
