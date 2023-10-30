use lexer::{lexer, Token};
mod lexer;
mod parser;
fn main() {
    let tokens = lexer();
    println!("{:?}", tokens.unwrap());
}

fn parser(tokens: Vec<Token>) {}
