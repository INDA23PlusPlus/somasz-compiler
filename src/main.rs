enum BinOpType {
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
enum UnanyOpType {
    Compl,
    Not,
}

enum TokenType {
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
struct Token {
    t_type: TokenType,
}
fn main() {}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
