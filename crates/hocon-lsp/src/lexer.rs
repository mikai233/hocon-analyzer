use std::{ops::Deref, range::Range};

use rowan::{TextRange, TextSize};

use crate::syntax::HoconSyntaxKind;

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
    cursor: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Token<'a> {
    pub(crate) token: HoconSyntaxKind,
    pub(crate) slice: &'a str,
    pub(crate) range: Range<usize>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(lexer: &mut logos::Lexer<'a, HoconSyntaxKind>) -> Self {
        let mut tokens = vec![];
        while let Some((syntax, range)) = lexer.spanned().next() {
            match syntax {
                Ok(syntax) => {
                    let ok = Token {
                        token: syntax,
                        slice: lexer.slice(),
                        range,
                    };
                    tokens.push(ok);
                }
                Err(_) => {
                    let err = Token {
                        token: HoconSyntaxKind::Error,
                        slice: lexer.slice(),
                        range,
                    };
                    tokens.push(err);
                }
            }
        }
        Self { tokens, cursor: 0 }
    }

    #[inline]
    pub(crate) fn current(&self) -> Option<&Token<'_>> {
        self.tokens.get(self.cursor)
    }

    #[inline]
    pub(crate) fn peek(&self) -> Option<&Token<'_>> {
        self.tokens.get(self.cursor + 1)
    }

    pub(crate) fn next(&mut self) -> Option<Token<'_>> {
        let token = self.tokens.get(self.cursor).cloned();
        self.bump();
        token
    }

    #[inline]
    pub(crate) fn bump(&mut self) {
        self.cursor += 1;
    }

    pub(crate) fn rewind(&mut self) {
        if self.cursor == 0 {
            panic!("cannot rewind when cursor is 0");
        } else {
            self.cursor -= 1;
        }
    }

    #[inline]
    pub(crate) fn get_cursor(&self) -> usize {
        self.cursor
    }
}

impl<'a> Deref for Lexer<'a> {
    type Target = Vec<Token<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}
