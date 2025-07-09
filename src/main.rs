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
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Identifier,
    If,
    Begin,
    End,
    WhiteSpace,
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

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn read_next_kind(&mut self) -> Kind {
        for c in self.chars.by_ref() {
            match c {
                '>' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Kind::GreaterThanEqual;
                    }
                    _ => return Kind::GreaterThan,
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Kind::LessThanEqual;
                    }
                    _ => return Kind::LessThan,
                },
                ' ' => return Kind::WhiteSpace,
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

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_eof() {
            let token = self.read_next_token();
            Some(token)
        } else {
            None
        }
    }
}

fn main() {
    let str = "if a < 10 begin print 'ok' end";
    let mut lexer = Lexer::new(str);
    loop {
        let t = lexer.read_next_token();
        println!("{:?}", t);
        if t.kind == Kind::Eof {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Kind, Lexer, Token};

    #[test]
    fn test_iterator() {
        let source = "< > <= >=";
        let lexer = Lexer::new(source);
        let tokens = lexer.collect::<Vec<Token>>();
        let expect = vec![
            Token {
                kind: Kind::LessThan,
                start: 0,
                end: 1,
            },
            Token {
                kind: Kind::WhiteSpace,
                start: 1,
                end: 2,
            },
            Token {
                kind: Kind::GreaterThan,
                start: 2,
                end: 3,
            },
            Token {
                kind: Kind::WhiteSpace,
                start: 3,
                end: 4,
            },
            Token {
                kind: Kind::LessThanEqual,
                start: 4,
                end: 6,
            },
            Token {
                kind: Kind::WhiteSpace,
                start: 6,
                end: 7,
            },
            Token {
                kind: Kind::GreaterThanEqual,
                start: 7,
                end: 9,
            },
        ];
        assert_eq!(expect, tokens);
    }
    #[test]
    fn test_basics() {
        let _source = "if a < 10 begin print 'ok' end";
        assert_eq!(true, true);
    }
    #[test]
    fn test_basics_more() {
        let _source = "
if @test = 1 begin
select 1
where 1=1
end
";
        assert_eq!(true, true);
    }
}
