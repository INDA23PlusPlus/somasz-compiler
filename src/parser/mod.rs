use crate::lexer::{BinOpType, Token, TokenType, UnanyOpType};

type BNode = Box<Node>;

#[derive(Debug, Clone)]
pub enum Node {
    TransaltionUnit(Vec<Node>),
    Identifier(String),
    Integer(i64),
    If(BNode, Vec<Node>, Vec<Node>),
    Assginment(BNode, BNode),
    Function(BNode, Vec<Node>, Vec<Node>),
    Return(BNode),
    BinExp(BNode, BinOpType, BNode),
    UnExp(BNode, UnanyOpType),
    FunctionCall(BNode, BNode),
}

pub fn parser(tokens: &Vec<Token>) -> Node {
    let mut nodes = vec![Node::Integer(10); 0];
    let mut cursor: usize = 0;

    while cursor < tokens.len() {
        match function(tokens, &mut cursor) {
            Some(node) => nodes.push(node),
            None => error_handler(),
        }
    }
    Node::TransaltionUnit(nodes)
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
        panic!();
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
                        nodes.push(if_statement(tokens, cursor)?)
                    }
                    TokenType::Return => {
                        *cursor += 1;
                        nodes.push(return_node(tokens, cursor)?)
                    }
                    TokenType::Let => {
                        *cursor += 1;
                        nodes.push(assginment(tokens, cursor)?)
                    }

                    _ => panic!(),
                }
            }
            *cursor += 1;
            Some(nodes)
        }
        _ => panic!(),
    }
}

fn if_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    let expression = match expression(tokens, cursor)? {
        Node::BinExp(r_node, op_type, l_node) => {
            // *cursor += 1;
            Some(Box::new(Node::BinExp(r_node, op_type, l_node)))
        }

        Node::UnExp(node, op_type) => {
            // *cursor += 1;

            Some(Box::new(Node::UnExp(node, op_type)))
        }

        _ => panic!(),
    };
    Some(Node::If(
        expression?,
        true_branch(tokens, cursor)?,
        false_branch(tokens, cursor)?,
    ))
}

fn assginment(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    match tokens[*cursor].t_type {
        TokenType::Identifier(_) => Some(Node::Assginment(
            Box::new(identifier(tokens, cursor)?),
            Box::new({
                *cursor += 1;
                match tokens[*cursor].t_type {
                    TokenType::Assginment => {
                        *cursor += 1;
                        match tokens[*cursor].t_type {
                            TokenType::Identifier(_) => identifier(tokens, cursor)?,
                            TokenType::Literal(_) => integer(tokens, cursor)?,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }),
        )),
        _ => panic!(),
    }
}

fn return_node(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    match tokens[*cursor].t_type {
        // TokenType::Identifier(_) => {
        //     let ret_value = Some(Node::Return(Box::new(expression(tokens, cursor)?)));
        //     match tokens[*cursor].t_type {
        //         TokenType::SemiColon => {
        //             *cursor += 1;
        //             ret_value
        //         }

        //         _ => panic!(),
        //     }
        // }
        // TokenType::Literal(_) => {
        //     let ret_value = Node::Return(Box::new(integer(tokens, cursor)?));
        //     match tokens[*cursor].t_type {
        //         TokenType::SemiColon => {
        //             *cursor += 1;

        //             Some(ret_value)
        //         }

        //         _ => panic!(),
        //     }
        // }
        TokenType::LeftPar => {
            let ret_value = Node::Return(Box::new(expression(tokens, cursor)?));
            match tokens[*cursor].t_type {
                TokenType::SemiColon => {
                    *cursor += 1;

                    Some(ret_value)
                }

                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn expression(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    match tokens[*cursor].t_type {
        TokenType::LeftPar => {
            *cursor += 1;
            let epxression = expression(tokens, cursor);
            *cursor += 1;
            epxression
        }
        TokenType::UnanyOp(_) => unary_expression(tokens, cursor),

        TokenType::Identifier(_) => {
            *cursor += 1;
            match tokens[*cursor].t_type {
                TokenType::LeftPar => {
                    *cursor -= 1;
                    funcall_expression(tokens, cursor)
                }
                TokenType::BinOp(op_type) => {
                    *cursor -= 1;
                    binary_expression(tokens, cursor, &op_type)
                }

                TokenType::SemiColon => {
                    *cursor -= 1;
                    identifier(tokens, cursor)
                }
                TokenType::LeftCurl => {
                    *cursor -= 1;
                    identifier(tokens, cursor)
                }
                TokenType::RightPar => {
                    *cursor -= 1;
                    identifier(tokens, cursor)
                }

                _ => panic!(),
            }
        }
        TokenType::Literal(_) => {
            *cursor += 1;
            match tokens[*cursor].t_type {
                TokenType::BinOp(op_type) => {
                    *cursor -= 1;
                    binary_expression(tokens, cursor, &op_type)
                }
                TokenType::SemiColon => {
                    *cursor -= 1;
                    integer(tokens, cursor)
                }
                TokenType::LeftCurl => {
                    *cursor -= 1;
                    integer(tokens, cursor)
                }
                TokenType::RightPar => {
                    *cursor -= 1;
                    integer(tokens, cursor)
                }

                _ => panic!(),
            }
        }

        // TokenType::Literal(_)
        // TokenType::BinOp(_) => {
        //     *cursor += 1;
        //     expression(tokens, cursor)
        // }
        _ => panic!(),
    }
}
fn binary_expression(tokens: &Vec<Token>, cursor: &mut usize, op_type: &BinOpType) -> Option<Node> {
    match tokens[*cursor].t_type {
        TokenType::Identifier(_) => Some(Node::BinExp(
            Box::new(identifier(tokens, cursor)?),
            op_type.to_owned(),
            {
                *cursor += 1;
                Box::new(expression(tokens, cursor)?)
            },
        )),
        TokenType::Literal(_) => Some(Node::BinExp(
            Box::new(integer(tokens, cursor)?),
            op_type.to_owned(),
            {
                *cursor += 1;
                Box::new(expression(tokens, cursor)?)
            },
        )),

        _ => panic!(),
    }
}

fn unary_expression(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    match tokens[*cursor].t_type {
        TokenType::UnanyOp(unany_op_type) => {
            *cursor += 1;
            Some(Node::UnExp(
                Box::new(expression(tokens, cursor)?),
                unany_op_type.to_owned(),
            ))
        }
        _ => panic!(),
    }
}
fn funcall_expression(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    Some(Node::FunctionCall(
        Box::new(identifier(tokens, cursor)?),
        Box::new(expression(tokens, cursor)?),
    ))
}

fn false_branch(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Vec<Node>> {
    match tokens[*cursor].t_type {
        TokenType::Else => {
            *cursor += 1;

            block_statement(tokens, cursor)
        }

        _ => panic!(),
    }
}

fn true_branch(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Vec<Node>> {
    block_statement(tokens, cursor)
}

fn argument(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Vec<Node>> {
    let mut nodes = vec![Node::Integer(10); 0];
    match tokens[*cursor].t_type {
        TokenType::LeftPar => {
            *cursor += 1;
            while match tokens[*cursor].t_type {
                TokenType::Identifier(_) => true,
                _ => false,
            } {
                nodes.push(identifier(tokens, cursor)?)
            }
            match tokens[*cursor].t_type {
                TokenType::RightPar => {
                    *cursor += 1;
                    Some(nodes)
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn identifier(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    if let TokenType::Identifier(x) = &tokens[*cursor].t_type {
        *cursor += 1;
        Some(Node::Identifier(x.to_string()))
    } else {
        panic!()
    }
}

fn integer(tokens: &Vec<Token>, cursor: &mut usize) -> Option<Node> {
    if let TokenType::Literal(x) = tokens[*cursor].t_type {
        *cursor += 1;
        Some(Node::Integer(x))
    } else {
        panic!()
    }
}

fn error_handler() {
    panic!();
}
