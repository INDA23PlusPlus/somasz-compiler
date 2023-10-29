use lexer::lexer;

mod lexer;

fn main() {
    let tokens = lexer();
    for i in tokens {
        println!("{:?}", i);
    }
}
