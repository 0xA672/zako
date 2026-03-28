#[derive(Debug, Clone, PartialEq)]
enum Token {
    String(&'a str),
    Number(f64),
    Bool(bool),
    Null,
    DotLBrace,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Equal,
    Ident(&'a str),     
    Const,
    Eof
}

struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    pos: usize,
    peek: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let peek = chars.next();
        Lexer {
            input,
            chars,
            pos: 0,
            peek,
        }
    }
    
    fn bump(&mut self) -> Option<char> {
        let c = self.peek?;
        self.peek = self.chars.next();
        self.pos += c.len_utf8();
        Some(c)
    }
    
            
}

