use std::borrow::Cow;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    String(Cow<'a, str>),
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
    inp: &'a [u8],
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            inp: s.as_bytes(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn bump(&mut self) -> Option<u8> {
        if self.pos < self.inp.len() {
            let b = self.inp[self.pos];
            self.pos += 1;
            if b == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(b)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<u8> {
        self.inp.get(self.pos).copied()
    }

    fn err<T>(&self, msg: String) -> Result<T, String> {
        Err(format!("{} at line {} col {}", msg, self.line, self.col))
    }

    fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            if b.is_ascii_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn skip_cmt(&mut self) -> Result<bool, String> {
        if self.peek() != Some(b'/') {
            return Ok(false);
        }
        match self.inp.get(self.pos + 1).copied() {
            Some(b'/') => {
                self.bump();
                self.bump();
                while let Some(b) = self.peek() {
                    if b == b'\n' {
                        self.bump();
                        break;
                    }
                    self.bump();
                }
                Ok(true)
            }
            Some(b'*') => {
                self.bump();
                self.bump();
                let mut d = 1;
                while d > 0 {
                    match self.peek() {
                        Some(b'*') => {
                            self.bump();
                            if self.peek() == Some(b'/') {
                                self.bump();
                                d -= 1;
                            }
                        }
                        Some(b'/') => {
                            self.bump();
                            if self.peek() == Some(b'*') {
                                self.bump();
                                d += 1;
                            }
                        }
                        Some(_) => {
                            self.bump();
                        }
                        None => return self.err("unclosed block comment".into()),
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn rd_id(&mut self) -> &'a str {
        let s = self.pos;
        if self.peek() == Some(b'.') {
            self.bump();
        }
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.bump();
            } else {
                break;
            }
        }
        str::from_utf8(&self.inp[s..self.pos]).unwrap()
    }

    fn rd_str(&mut self) -> Result<Cow<'a, str>, String> {
        self.bump(); // '"'
        let mut v = Vec::new();
        let s = self.pos;
        while let Some(b) = self.peek() {
            if b == b'"' {
                let r = if v.is_empty() {
                    Cow::Borrowed(str::from_utf8(&self.inp[s..self.pos]).unwrap())
                } else {
                    v.extend_from_slice(&self.inp[s..self.pos]);
                    Cow::Owned(String::from_utf8_lossy(&v).into_owned())
                };
                self.bump();
                return Ok(r);
            }
            if b == b'\\' {
                v.extend_from_slice(&self.inp[s..self.pos]);
                self.bump();
                match self.bump() {
                    Some(b'"') => v.push(b'"'),
                    Some(b'\\') => v.push(b'\\'),
                    Some(b'n') => v.push(b'\n'),
                    Some(b'r') => v.push(b'\r'),
                    Some(b't') => v.push(b'\t'),
                    Some(c) => return self.err(format!("unknown escape \\{}", c as char)),
                    None => return self.err("unclosed string escape".into()),
                }
                continue;
            }
            self.bump();
        }
        self.err("unclosed string".into())
    }

    fn rd_num(&mut self) -> Result<f64, String> {
        let s = self.pos;
        if self.peek() == Some(b'-') {
            self.bump();
        }
        while let Some(b) = self.peek() {
            if b.is_ascii_digit() {
                self.bump();
            } else {
                break;
            }
        }
        if self.peek() == Some(b'.') {
            self.bump();
            while let Some(b) = self.peek() {
                if b.is_ascii_digit() {
                    self.bump();
                } else {
                    break;
                }
            }
        }
        if let Some(b) = self.peek() {
            if b == b'e' || b == b'E' {
                self.bump();
                if let Some(b'+') | Some(b'-') = self.peek() {
                    self.bump();
                }
                while let Some(b) = self.peek() {
                    if b.is_ascii_digit() {
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
        }
        let slice = &self.inp[s..self.pos];
        let txt = str::from_utf8(slice).unwrap();
        txt.parse::<f64>().map_err(|_| format!("invalid number '{}'", txt))
    }

    pub fn next(&mut self) -> Result<Token<'a>, String> {
        loop {
            self.skip_ws();
            if !self.skip_cmt()? {
                break;
            }
        }
        let b = match self.peek() {
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
                if self.inp.get(self.pos + 1) == Some(&b'{') {
                    self.bump();
                    self.bump();
                    Ok(Token::DotLBrace)
                } else {
                    let id = self.rd_id();
                    Ok(Token::Ident(id))
                }
            }
            b'"' => {
                let s = self.rd_str()?;
                Ok(Token::String(s))
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let id = self.rd_id();
                match id {
                    "true" => Ok(Token::Bool(true)),
                    "false" => Ok(Token::Bool(false)),
                    "null" => Ok(Token::Null),
                    "const" => Ok(Token::Const),
                    _ => Ok(Token::Ident(id)),
                }
            }
            b'0'..=b'9' | b'-' => {
                let n = self.rd_num()?;
                Ok(Token::Number(n))
            }
            _ => self.err(format!("unexpected char '{}'", b as char)),
        }
    }
}
