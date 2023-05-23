use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input files to scan
    file: Vec<String>,
}

#[derive(Debug)]
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
    LeftParen,
    RightParen,

    LeftCurly,
    RightCurly,

    LeftSquare,
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

    // Other Stuff
    Begin,
    End,
    String(String),
    RegExp(String),
    Other(String),
}

struct Lexer {
    chars: Vec<char>,
    idx: usize,
}

impl Lexer {
    fn new(str: String) -> Self {
        Self {
            chars: str.trim().chars().collect(),
            idx: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        let mut word = String::new();

        while self.idx < self.chars.len() {
            if self.is_whitespace() {
                self.skip_whitespace();
                break;
            } else if self.chars[self.idx] == '#' {
                self.skip_comment_line();
            } else if self.chars[self.idx] == '"' {
                self.idx += 1; // skip first '"'
                let str = self.handle_string();
                self.skip_whitespace();
                return Some(Token::String(str));
            } else if self.chars[self.idx] == '/' {
                self.idx += 1; // skip first '/'
                let re = self.handle_regex();
                self.skip_whitespace();
                return Some(Token::RegExp(re));
            } else {
                word.push(self.chars[self.idx]);
                self.idx += 1;
            }
        }

        if self.idx > self.chars.len() {
            return None;
        }

        // println!("{word}");
        match &word[..] {
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "function" => Some(Token::Function),
            "in" => Some(Token::In),
            "do" => Some(Token::Do),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
            "print" => Some(Token::Print),
            "printf" => Some(Token::Printf),
            "return" => Some(Token::Return),
            "next" => Some(Token::Next),
            "nextfile" => Some(Token::Nextfile),
            "delete" => Some(Token::Delete),
            "exit" => Some(Token::Exit),

            "(" => Some(Token::LeftParen),
            ")" => Some(Token::RightParen),

            "{" => Some(Token::LeftCurly),
            "}" => Some(Token::RightCurly),

            "[" => Some(Token::LeftSquare),
            "]" => Some(Token::RightSquare),

            "+" => Some(Token::Plus),
            "-" => Some(Token::Minus),
            "*" => Some(Token::Multiply),
            "/" => Some(Token::Divide),
            "%" => Some(Token::Modulus),
            "^" => Some(Token::Pow),
            "++" => Some(Token::DoublePlus),
            "--" => Some(Token::DoubleMinus),
            "+=" => Some(Token::PlusEqual),
            "-=" => Some(Token::MinusEqual),
            "*=" => Some(Token::TimesEqual),
            "/=" => Some(Token::DivEqual),
            "%=" => Some(Token::ModEqual),
            "^=" => Some(Token::PowEqual),

            "!" => Some(Token::Not),
            ">" => Some(Token::Greater),
            ">=" => Some(Token::GreaterEqual),
            "<" => Some(Token::Less),
            "<=" => Some(Token::LessEqual),
            "==" => Some(Token::Equal),
            "!=" => Some(Token::NotEqual),

            "BEGIN" => Some(Token::Begin),
            "END" => Some(Token::End),

            // word => Some(Token::Other(word.to_string())),
            _ => None,
        }
    }

    fn peek(&self) -> Option<Token> {
        Some(Token::Return)
    }

    fn is_whitespace(&self) -> bool {
        if let Some(c) = self.chars.get(self.idx) {
            c == &' ' || c == &'\n' || c == &'\t'
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.idx += 1;
        }
    }

    fn skip_comment_line(&mut self) {
        while let Some(c) = self.chars.get(self.idx) {
            if c == &'\n' {
                break;
            }
            self.idx += 1;
        }
    }

    fn handle_string(&mut self) -> String {
        let mut str = String::new();

        while self.idx < self.chars.len() {
            let c = self.chars[self.idx];
            if c == '\\' {
                if self.chars[self.idx + 1] == '"' {
                    str.push('\\');
                    str.push('"');
                    self.idx += 2; // 1 for \ and 1 for "
                    continue;
                }
            }
            if c == '"' {
                self.idx += 1; // skip closing '"'
                break;
            }
            str.push(c);
            self.idx += 1;
        }

        str
    }

    fn handle_regex(&mut self) -> String {
        let mut str = String::new();

        while self.idx < self.chars.len() {
            let c = self.chars[self.idx];
            if c == '/' {
                self.idx += 1; // skip ending '/'
                break;
            }
            str.push(c);
            self.idx += 1;
        }

        str
    }
}

fn main() {
    let _args = Args::parse();

    // let awk = r#" { } "abc\"def\"ghi" ( ) [ ] function BEGIN END if else while do exit print + ++ -="#;

    let awk = r#"
BEGIN { print "Analysis of \"li\"" }
/li/  { ++n }
END   { print "\"li\" appears in", n, "records." }
        "#;

    let mut lex = Lexer::new(awk.to_string());
    while let Some(token) = lex.next() {
        println!("{:?}", token);
    }
}
