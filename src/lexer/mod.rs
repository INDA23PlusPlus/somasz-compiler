use std::fs::File;
use std::fs::*;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]

pub enum BinOpType {
    Plus,
    Minus,
    Modulo,
    Devide,
    Multiply,
    BitOr,
    BitAnd,
    BitXor,
    Equ,
    Nequ,
    Greater,
    Less,
    LessEq,
    GreaterEq,
    LogAnd,
    LogOr,
}
#[derive(Debug, Clone)]

pub enum UnanyOpType {
    Compl,
    Not,
}

#[derive(Debug, Clone)]

pub enum TokenType {
    UnanyOp(UnanyOpType),
    BinOp(BinOpType),
    Literal(i64),
    Identifier(String),
    Assginment,
    Let,
    If,
    Else,
    While,
    Fn,
    Return,
    LeftPar,
    RightPar,
    LeftCurl,
    RightCurl,
    SemiColon,
    Comma,
}
//Lib c for print
#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
}
pub fn lexer() -> std::io::Result<Vec<Token>> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut pointer: usize = 0;
    let mut tokens = vec![
        Token {
            t_type: TokenType::SemiColon,
        };
        0
    ];
    while pointer < contents.len() {
        let character = &contents[pointer..pointer + 1];
        if char::is_alphabetic(character.chars().collect::<Vec<_>>()[0]) {
            match character {
                "f" => {
                    if &contents[pointer + 1..pointer + 2] == "n" {
                        tokens.push(Token {
                            t_type: TokenType::Fn,
                        });
                        pointer += 2;
                    } else {
                        let (token, retrun_contents, retrun_pointer) =
                            create_identifier_token(contents, pointer);
                        contents = retrun_contents;
                        pointer = retrun_pointer;
                        tokens.push(token)
                    }
                }
                "l" => {
                    if &contents[pointer + 1..pointer + 2] == "e"
                        && &contents[pointer + 2..pointer + 3] == "t"
                    {
                        tokens.push(Token {
                            t_type: TokenType::Let,
                        });
                        pointer += 3;
                    } else {
                        let (token, retrun_contents, retrun_pointer) =
                            create_identifier_token(contents, pointer);
                        contents = retrun_contents;
                        pointer = retrun_pointer;
                        tokens.push(token)
                    }
                }
                "i" => {
                    if &contents[pointer + 1..pointer + 2] == "f" {
                        tokens.push(Token {
                            t_type: TokenType::If,
                        });
                        pointer += 2;
                    } else {
                        let (token, retrun_contents, retrun_pointer) =
                            create_identifier_token(contents, pointer);
                        contents = retrun_contents;
                        pointer = retrun_pointer;
                        tokens.push(token)
                    }
                }
                "e" => {
                    if &contents[pointer + 1..pointer + 2] == "l"
                        && &contents[pointer + 2..pointer + 3] == "s"
                        && &contents[pointer + 3..pointer + 4] == "e"
                    {
                        tokens.push(Token {
                            t_type: TokenType::Else,
                        });
                        pointer += 4;
                    } else {
                        let (token, retrun_contents, retrun_pointer) =
                            create_identifier_token(contents, pointer);
                        contents = retrun_contents;
                        pointer = retrun_pointer;
                        tokens.push(token)
                    }
                }
                "w" => {
                    if &contents[pointer + 1..pointer + 2] == "h"
                        && &contents[pointer + 2..pointer + 3] == "i"
                        && &contents[pointer + 3..pointer + 4] == "l"
                        && &contents[pointer + 4..pointer + 5] == "e"
                    {
                        tokens.push(Token {
                            t_type: TokenType::While,
                        });
                        pointer += 5;
                    } else {
                        let (token, retrun_contents, retrun_pointer) =
                            create_identifier_token(contents, pointer);
                        contents = retrun_contents;
                        pointer = retrun_pointer;
                        tokens.push(token)
                    }
                }
                "r" => {
                    if &contents[pointer + 1..pointer + 2] == "e"
                        && &contents[pointer + 2..pointer + 3] == "t"
                        && &contents[pointer + 3..pointer + 4] == "u"
                        && &contents[pointer + 4..pointer + 5] == "r"
                        && &contents[pointer + 5..pointer + 6] == "n"
                    {
                        tokens.push(Token {
                            t_type: TokenType::Return,
                        });
                        pointer += 6;
                    } else {
                        let (token, retrun_contents, retrun_pointer) =
                            create_identifier_token(contents, pointer);
                        contents = retrun_contents;
                        pointer = retrun_pointer;
                        tokens.push(token)
                    }
                }

                _ => {
                    let (token, retrun_contents, retrun_pointer) =
                        create_identifier_token(contents, pointer);
                    contents = retrun_contents;
                    pointer = retrun_pointer;
                    tokens.push(token)
                }
            }
        } else if char::is_numeric(character.chars().collect::<Vec<_>>()[0]) {
            let mut literl_vaue = String::new();
            while char::is_numeric(contents[pointer..pointer + 1].chars().collect::<Vec<_>>()[0]) {
                literl_vaue.push_str(&contents[pointer..pointer + 1]);
                pointer += 1;
            }
            tokens.push(Token {
                t_type: TokenType::Literal(literl_vaue.to_string().parse::<i64>().unwrap()),
            })
        } else if char::is_whitespace(character.chars().collect::<Vec<_>>()[0]) {
            pointer += 1;
        } else if char::is_ascii_punctuation(&character.chars().collect::<Vec<_>>()[0]) {
            match character {
                ";" => {
                    tokens.push(Token {
                        t_type: TokenType::SemiColon,
                    });
                    pointer += 1
                }

                "=" => {
                    if &contents[pointer + 1..pointer + 2] == "=" {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::Equ),
                        });
                        pointer += 2
                    } else {
                        tokens.push(Token {
                            t_type: TokenType::Assginment,
                        });
                        pointer += 1
                    }
                }
                "(" => {
                    tokens.push(Token {
                        t_type: TokenType::LeftPar,
                    });
                    pointer += 1
                }
                ")" => {
                    tokens.push(Token {
                        t_type: TokenType::RightPar,
                    });
                    pointer += 1
                }
                "{" => {
                    tokens.push(Token {
                        t_type: TokenType::LeftCurl,
                    });
                    pointer += 1
                }
                "}" => {
                    tokens.push(Token {
                        t_type: TokenType::RightCurl,
                    });
                    pointer += 1
                }
                "," => {
                    tokens.push(Token {
                        t_type: TokenType::Comma,
                    });
                    pointer += 1
                }
                "!" => {
                    if &contents[pointer + 1..pointer + 2] == "=" {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::Nequ),
                        });
                        pointer += 2;
                    } else {
                        tokens.push(Token {
                            t_type: TokenType::UnanyOp(UnanyOpType::Not),
                        });
                        pointer += 1
                    }
                }
                "~" => {
                    tokens.push(Token {
                        t_type: TokenType::UnanyOp(UnanyOpType::Compl),
                    });
                    pointer += 1
                }
                "+" => {
                    tokens.push(Token {
                        t_type: TokenType::BinOp(BinOpType::Plus),
                    });
                    pointer += 1
                }
                "-" => {
                    tokens.push(Token {
                        t_type: TokenType::BinOp(BinOpType::Minus),
                    });
                    pointer += 1
                }
                "%" => {
                    tokens.push(Token {
                        t_type: TokenType::BinOp(BinOpType::Modulo),
                    });
                    pointer += 1
                }
                "/" => {
                    tokens.push(Token {
                        t_type: TokenType::BinOp(BinOpType::Devide),
                    });
                    pointer += 1
                }
                "*" => {
                    tokens.push(Token {
                        t_type: TokenType::BinOp(BinOpType::Multiply),
                    });
                    pointer += 1
                }
                ">" => {
                    if &contents[pointer + 1..pointer + 2] == "=" {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::GreaterEq),
                        });
                        pointer += 2
                    } else {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::Greater),
                        });
                        pointer += 1
                    }
                }
                "<" => {
                    if &contents[pointer + 1..pointer + 2] == "=" {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::LessEq),
                        });
                        pointer += 2
                    } else {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::Less),
                        });
                        pointer += 1
                    }
                }
                "&" => {
                    if &contents[pointer + 1..pointer + 2] == "&" {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::LogAnd),
                        });
                        pointer += 2
                    } else {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::BitAnd),
                        });
                        pointer += 1
                    }
                }
                "|" => {
                    if &contents[pointer + 1..pointer + 2] == "|" {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::LogOr),
                        });
                        pointer += 2
                    } else {
                        tokens.push(Token {
                            t_type: TokenType::BinOp(BinOpType::BitOr),
                        });
                        pointer += 1
                    }
                }
                "^" => {
                    tokens.push(Token {
                        t_type: TokenType::BinOp(BinOpType::BitXor),
                    });
                    pointer += 1
                }

                _ => pointer += 1,
            }
        }
    }

    Ok(tokens)
}

fn create_identifier_token(contents: String, mut pointer: usize) -> (Token, String, usize) {
    let mut identifier_string = String::new();
    while char::is_alphanumeric(contents[pointer..pointer + 1].chars().collect::<Vec<_>>()[0])
        || &contents[pointer..pointer + 1] == "_"
    {
        identifier_string.push_str(&contents[pointer..pointer + 1]);
        pointer += 1;
    }

    (
        Token {
            t_type: TokenType::Identifier(identifier_string),
        },
        contents,
        pointer,
    )
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
