use lexer::lexer;
use parser::parser;
mod lexer;
mod parser;
fn main() {
    let tokens = lexer().unwrap();
    println!("{:?}", tokens);

    let ast = parser(&tokens);

    println!("{:?}", ast);
}
