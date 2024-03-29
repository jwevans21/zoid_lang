#![deny(
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unimplemented,
    clippy::todo
)]

use std::str::Chars;

use zoid_error::{panic, ZoidError, ZoidErrorCode, ZoidErrorSeverity};

#[cfg(test)]
mod test;
mod token;

pub use token::{ZoidToken, ZoidTokenKind};

#[derive(Debug, Clone)]
pub struct ZoidLexer<'a, 'b> {
    source: &'a str,
    chars: Chars<'a>,
    file: &'b str,
    position: usize,
    line: u32,
    column: u32,
}

impl<'a, 'b> ZoidLexer<'a, 'b> {
    pub fn new(source: &'a str, file: &'b str) -> Self {
        Self {
            source,
            chars: source.chars(),
            file,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn lex(&mut self) -> Result<Option<ZoidToken<'a>>, ZoidError<'a, 'b>> {
        self.skip_whitespace();

        let start = self.position;
        let c = match self.next() {
            Some(c) => c,
            None => return Ok(None),
        };

        let kind = match c {
            '+' => ZoidTokenKind::OpAdd,
            '-' => ZoidTokenKind::OpSub,
            '*' => ZoidTokenKind::OpMul,
            '/' => ZoidTokenKind::OpDiv,
            '%' => ZoidTokenKind::OpRem,
            '&' => match self.peek() {
                Some('&') => {
                    self.next();
                    ZoidTokenKind::OpLogAnd
                }
                _ => ZoidTokenKind::OpBitAnd,
            },
            '|' => match self.peek() {
                Some('|') => {
                    self.next();
                    ZoidTokenKind::OpLogOr
                }
                _ => ZoidTokenKind::OpBitOr,
            },
            '!' => match self.peek() {
                Some('=') => {
                    self.next();
                    ZoidTokenKind::OpNe
                }
                _ => ZoidTokenKind::OpLogNot,
            },
            '~' => ZoidTokenKind::OpBitNot,
            '^' => ZoidTokenKind::OpBitXor,
            '<' => match self.peek() {
                Some('<') => {
                    self.next();
                    ZoidTokenKind::OpBitShl
                }
                Some('=') => {
                    self.next();
                    ZoidTokenKind::OpLe
                }
                _ => ZoidTokenKind::OpLt,
            },
            '>' => match self.peek() {
                Some('>') => {
                    self.next();
                    ZoidTokenKind::OpBitShr
                }
                Some('=') => {
                    self.next();
                    ZoidTokenKind::OpGe
                }
                _ => ZoidTokenKind::OpGt,
            },
            '=' => match self.peek() {
                Some('=') => {
                    self.next();
                    ZoidTokenKind::OpEq
                }
                _ => ZoidTokenKind::OpAssign,
            },
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(start)?,
            '0'..='9' => self.lex_numeric(start)?,
            _ => {
                return Err(ZoidError {
                    severity: ZoidErrorSeverity::Error,
                    error_code: ZoidErrorCode::SyntaxError,
                    file: self.file,
                    line: self.line,
                    column: self.column,
                    source: self.source,
                })
            }
        };

        let end = self.position;

        Ok(Some(ZoidToken {
            kind,
            start,
            end,
            source: match self.source.get(start..end) {
                Some(s) => s,
                None => panic!("encountered invalid token range"),
            },
        }))
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(c) = c {
            self.position += c.len_utf8();
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn lex_identifier(&mut self, start: usize) -> Result<ZoidTokenKind, ZoidError<'a, 'b>> {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    self.next();
                }
                _ => break,
            }
        }

        let end = self.position;

        Ok(match self.source.get(start..end) {
            Some("if") => ZoidTokenKind::KwIf,
            Some("else") => ZoidTokenKind::KwElse,
            Some("while") => ZoidTokenKind::KwWhile,
            Some("let") => ZoidTokenKind::KwLet,
            Some("fn") => ZoidTokenKind::KwFn,
            Some("return") => ZoidTokenKind::KwReturn,
            Some(_) => ZoidTokenKind::Identifier,
            None => panic!("encountered invalid identifier range"),
        })
    }

    fn lex_numeric(&mut self, start: usize) -> Result<ZoidTokenKind, ZoidError<'a, 'b>> {
        let mut is_float = false;

        // Parse integer part
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.next();
                }
                _ => break,
            }
        }
        // Parse fractional part if present
        if let Some('.') = self.peek() {
            is_float = true;
            self.next();
            while let Some(c) = self.peek() {
                match c {
                    '0'..='9' => {
                        self.next();
                    }
                    _ => break,
                }
            }
        }
        // Parse exponent part if present
        if let Some('e') | Some('E') = self.peek() {
            is_float = true;
            self.next();
            if let Some('+') | Some('-') = self.peek() {
                self.next();
            }
            while let Some(c) = self.peek() {
                match c {
                    '0'..='9' => {
                        self.next();
                    }
                    _ => break,
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            let src = self.source.get(start..self.position);
            assert!(src.is_some());
            let src = src.unwrap();
            if is_float {
                assert!(src.parse::<f64>().is_ok());
            } else {
                assert!(src.parse::<i64>().is_ok());
            }
        }

        if is_float {
            Ok(ZoidTokenKind::Float)
        } else {
            Ok(ZoidTokenKind::Integer)
        }
    }
}
