use clap::Parser;

#[derive(Parser, Debug)]
struct Args {

    /// Input files to scan
    file: Vec<String>
}

enum Token {
    // Keywords
    If,
    Else,
    While,
    For,
    Function,
    In,
    Do,
    Break,
    Continue,
    Print,
    Printf,
    Return,
    Next,
    Nextfile,
    Delete,
    Exit,

    // Braces
    RightParen,
    LeftParen,
    RightCurly,
    LeftCurly,
    RightSquare,

    // Math symbols,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Pow,
    DoublePlus,
    DoubleMinus,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivEqual,
    ModEqual,
    PowEqual,

    // Logical symbols,
    Not,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,

    // ?:
}

struct Lexer {
    str: String,
    idx: usize,
}

impl Lexer {
    fn next(&mut self) -> Token {
        Token::Return
    }

    fn peek(&self) -> Token {
        Token::Return
    }

    fn skip_whitespcae(&mut self) {
    }
}

fn main() {
    let _args = Args::parse();
}
