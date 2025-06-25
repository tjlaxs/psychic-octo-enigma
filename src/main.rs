use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof,
    Plus,
    Minus,
}

struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }

    fn read_next_kind(&mut self) -> Kind {
        for c in self.chars.by_ref() {
            match c {
                '+' => return Kind::Plus,
                '-' => return Kind::Minus,
                _ => {}
            }
        }
        Kind::Eof
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token { kind, start, end }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}

fn main() {
    let str = "+-++--+";
    let mut lexer = Lexer::new(str);
    loop {
        let t = lexer.read_next_token();
        println!("{:?}", t);
        if t.kind == Kind::Eof {
            break;
        }
    }
}
