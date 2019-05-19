use std::cell::RefCell;
use std::cmp::{max, min};
use std::ops::FnMut;
use std::str::from_utf8;

type Result<T> = std::result::Result<T, LexError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        assert!(max(self.0, other.0) <= min(self.1, other.1));
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    /// [0-9]+
    Number(u64),
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// (
    LParen,
    /// )
    RParen,
}

type Token = Annot<TokenKind>;

impl Token {
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }
    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }
    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }
    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lexer<'a> {
    input: &'a [u8],
    pos: RefCell<usize>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: RefCell::new(0),
        }
    }

    fn lex(&self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        macro_rules! lex_a_token {
            ($lexer:expr) => {{
                let tok = $lexer?;
                tokens.push(tok);
            }};
        }

        loop {
            let pos = *self.pos.borrow();
            if self.input.len() <= pos {
                break;
            }

            match self.input[pos] {
                b'0'...b'9' => lex_a_token!(self.lex_number()),
                b'+' => lex_a_token!(self.lex_plus()),
                b'-' => lex_a_token!(self.lex_minus()),
                b'*' => lex_a_token!(self.lex_asterisk()),
                b'/' => lex_a_token!(self.lex_slash()),
                b'(' => lex_a_token!(self.lex_lparen()),
                b')' => lex_a_token!(self.lex_rparen()),
                b' ' | b'\n' | b'\t' => self.skip_spaces()?,
                b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
            }
        }

        Ok(tokens)
    }

    fn consume_byte(&self, b: u8) -> Result<(u8, usize)> {
        let mut pos = self.pos.borrow_mut();
        if self.input.len() <= *pos {
            return Err(LexError::eof(Loc(*pos, *pos)));
        }
        if self.input[*pos] != b {
            return Err(LexError::invalid_char(
                self.input[*pos] as char,
                Loc(*pos, *pos + 1),
            ));
        }

        *pos += 1;

        Ok((b, *pos))
    }

    fn recognize_many(&self, mut f: impl FnMut(u8) -> bool) -> usize {
        let mut pos = self.pos.borrow_mut();
        while *pos < self.input.len() && f(self.input[*pos]) {
            *pos += 1;
        }
        *pos
    }

    fn lex_plus(&self) -> Result<Token> {
        self.consume_byte(b'+')
            .map(|(_, end)| Token::plus(Loc(end - 1, end)))
    }
    fn lex_minus(&self) -> Result<Token> {
        self.consume_byte(b'-')
            .map(|(_, end)| Token::minus(Loc(end - 1, end)))
    }
    fn lex_asterisk(&self) -> Result<Token> {
        self.consume_byte(b'*')
            .map(|(_, end)| Token::asterisk(Loc(end - 1, end)))
    }
    fn lex_slash(&self) -> Result<Token> {
        self.consume_byte(b'/')
            .map(|(_, end)| Token::slash(Loc(end - 1, end)))
    }
    fn lex_lparen(&self) -> Result<Token> {
        self.consume_byte(b'(')
            .map(|(_, end)| Token::lparen(Loc(end - 1, end)))
    }
    fn lex_rparen(&self) -> Result<Token> {
        self.consume_byte(b')')
            .map(|(_, end)| Token::rparen(Loc(end - 1, end)))
    }

    fn lex_number(&self) -> Result<Token> {
        let start = *self.pos.borrow();
        let end = self.recognize_many(|b| b"0123456789".contains(&b));

        let n = from_utf8(&self.input[start..end]).unwrap().parse().unwrap();

        Ok(Token::number(n, Loc(start, end)))
    }

    fn skip_spaces(&self) -> Result<()> {
        self.recognize_many(|b| b" \n\t".contains(&b));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let lexer = Lexer::new("1 + 2 * 3 - -10");
        assert_eq!(
            lexer.lex(),
            Ok(vec![
                Token::number(1, Loc(0, 1)),
                Token::plus(Loc(2, 3)),
                Token::number(2, Loc(4, 5)),
                Token::asterisk(Loc(6, 7)),
                Token::number(3, Loc(8, 9)),
                Token::minus(Loc(10, 11)),
                Token::minus(Loc(12, 13)),
                Token::number(10, Loc(13, 15)),
            ])
        )
    }
}
