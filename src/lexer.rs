#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
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
    Eof,
    Semicolon,
}

pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn bump(&mut self) -> Option<u8> {
        if self.pos < self.input.len() {
            let byte = self.input[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn peekbyte(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn skipwhitespace(&mut self) {
        while let Some(b) = self.peekbyte() {
            if b.is_ascii_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn skipcomment(&mut self) -> bool {
        if self.peekbyte() != Some(b'/') {
            return false;
        }
        match self.input.get(self.pos + 1).copied() {
            Some(b'/') => {
                self.bump();
                self.bump();
                while let Some(b) = self.peekbyte() {
                    if b == b'\n' {
                        self.bump();
                        break;
                    }
                    self.bump();
                }
                true
            }
            Some(b'*') => {
                self.bump();
                self.bump();
                let mut depth = 1;
                while depth > 0 {
                    match self.peekbyte() {
                        Some(b'*') => {
                            self.bump();
                            if self.peekbyte() == Some(b'/') {
                                self.bump();
                                depth -= 1;
                            }
                        }
                        Some(b'/') => {
                            self.bump();
                            if self.peekbyte() == Some(b'*') {
                                self.bump();
                                depth += 1;
                            }
                        }
                        Some(_) => {
                            self.bump();
                        }
                        None => break,
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn read_ident(&mut self) -> &'a str {
        let start = self.pos;
        // Zig ZON 格式中，字段名可以 . 开头，如 .name、.url
        if self.peekbyte() == Some(b'.') {
            self.bump();
        }
        while let Some(b) = self.peekbyte() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.bump();
            } else {
                break;
            }
        }
        std::str::from_utf8(&self.input[start..self.pos]).unwrap()
    }

    fn read_string(&mut self) -> Result<&'a str, String> {
        self.bump();
        let start = self.pos;
        while let Some(b) = self.peekbyte() {
            if b == b'"' {
                let result = std::str::from_utf8(&self.input[start..self.pos]).unwrap();
                self.bump();
                return Ok(result);
            }
            if b == b'\\' {
                return Err("Escape sequences not supported in zero-copy mode".into());
            }
            self.bump();
        }
        Err("Unclosed string".into())
    }

    fn read_number(&mut self) -> f64 {
        let start = self.pos;
        if self.peekbyte() == Some(b'-') {
            self.bump();
        }
        while let Some(b) = self.peekbyte() {
            if b.is_ascii_digit() {
                self.bump();
            } else {
                break;
            }
        }
        if self.peekbyte() == Some(b'.') {
            self.bump();
            while let Some(b) = self.peekbyte() {
                if b.is_ascii_digit() {
                    self.bump();
                } else {
                    break;
                }
            }
        }
        if let Some(b) = self.peekbyte() {
            if b == b'e' || b == b'E' {
                self.bump();
                if let Some(b'+') | Some(b'-') = self.peekbyte() {
                    self.bump();
                }
                while let Some(b) = self.peekbyte() {
                    if b.is_ascii_digit() {
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
        }
        let slice = &self.input[start..self.pos];
        std::str::from_utf8(slice).unwrap().parse().unwrap()
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, String> {
        loop {
            self.skipwhitespace();
            if !self.skipcomment() {
                break;
            }
        }
        let b = match self.peekbyte() {
            Some(b) => b,
            None => return Ok(Token::Eof),
        };

        match b {
            b'{' => {
                self.bump();
                Ok(Token::LBrace)
            }
            b'}' => {
                self.bump();
                Ok(Token::RBrace)
            }
            b'[' => {
                self.bump();
                Ok(Token::LBracket)
            }
            b']' => {
                self.bump();
                Ok(Token::RBracket)
            }
            b':' => {
                self.bump();
                Ok(Token::Colon)
            }
            b';' => {
                self.bump();
                Ok(Token::Semicolon)
            }
            b',' => {
                self.bump();
                Ok(Token::Comma)
            }
            b'=' => {
                self.bump();
                Ok(Token::Equal)
            }
            b'.' => {
                self.bump();
                if self.peekbyte() == Some(b'{') {
                    self.bump();
                    Ok(Token::DotLBrace)
                } else if let Some(b) = self.peekbyte() {
                    // 如果 . 后面是字母或下划线，则是 ZON 字段名
                    if b.is_ascii_alphabetic() || b == b'_' {
                        let ident = self.read_ident();
                        Ok(Token::Ident(ident))
                    } else {
                        Err("Unexpected '.'".into())
                    }
                } else {
                    Err("Unexpected '.'".into())
                }
            }
            b'"' => {
                let s = self.read_string()?;
                Ok(Token::String(s))
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                match ident {
                    "true" => Ok(Token::Bool(true)),
                    "false" => Ok(Token::Bool(false)),
                    "null" => Ok(Token::Null),
                    "const" => Ok(Token::Const),
                    _ => Ok(Token::Ident(ident)),
                }
            }
            b'0'..=b'9' | b'-' => {
                let num = self.read_number();
                Ok(Token::Number(num))
            }
            _ => Err(format!("Unexpected byte: {}", b)),
        }
    }
}

