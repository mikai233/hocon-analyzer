use logos::Logos;
use rowan::GreenNodeBuilder;

use crate::{
    error::Error,
    lexer::{Lexer, Token},
    node_scope,
    syntax::HoconSyntaxKind,
};

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    lexer: Lexer<'a>,
    builder: GreenNodeBuilder<'a>,
    errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    #[inline]
    pub fn token(&mut self, kind: HoconSyntaxKind, text: &str) {
        self.builder.token(kind.into(), text);
    }

    pub(crate) fn parse(source: &str) {
        let mut lexer = HoconSyntaxKind::lexer(source);
        let lexer = Lexer::new(&mut lexer);
        let mut parser = Parser {
            lexer,
            builder: GreenNodeBuilder::new(),
            errors: vec![],
        };
        node_scope!(
            parser.builder,
            HoconSyntaxKind::Root,
            parser.parse_root_object()
        );
    }

    fn parse_whitespace(&mut self) {}

    fn parse_root_object(&mut self) {
        while let Some(Token {
            token: syntax,
            slice,
        }) = self.lexer.next()
        {
            match syntax {
                HoconSyntaxKind::HorizontalWhitespace | HoconSyntaxKind::LineEnding => {
                    self.builder.token(syntax.into(), slice);
                }
                HoconSyntaxKind::Ident => {
                    self.lexer.rewind();
                    self.parse_object_entries();
                }
                HoconSyntaxKind::LeftBrace => {
                    self.builder.token(syntax.into(), slice);
                }
                HoconSyntaxKind::RightBrace => {
                    self.builder.token(syntax.into(), slice);
                }
                HoconSyntaxKind::HashComment | HoconSyntaxKind::SlashComment => {
                    self.builder.token(syntax.into(), slice);
                }
                HoconSyntaxKind::Error => todo!(),
                _ => {}
            }
        }
    }

    fn parse_object(&mut self) {
        node_scope!(self.builder, HoconSyntaxKind::Object, {});
    }

    fn parse_key(&mut self) {
        let point = self.builder.checkpoint();
        let mut point_started = false;
        while let Some(Token {
            token: syntax,
            slice,
        }) = self.lexer.next()
        {
            match syntax {
                HoconSyntaxKind::Ident | HoconSyntaxKind::HorizontalWhitespace => {
                    self.builder.token(syntax.into(), slice);
                }
                HoconSyntaxKind::Dot => {
                    if !point_started {
                        point_started = true;
                        self.builder
                            .start_node_at(point, HoconSyntaxKind::PathExpression.into());
                        self.builder.token(syntax.into(), slice);
                    }
                }
                _ => {
                    self.lexer.rewind();
                    break;
                }
            }
        }
        if point_started {
            self.builder.finish_node();
        }
    }

    fn parse_value(&mut self) {}

    fn parse_object_entries(&mut self) {
        node_scope!(self.builder, HoconSyntaxKind::Key, self.parse_key());
        self.parse_whitespace();
        match self.lexer.next() {
            Some(Token {
                token: syntax,
                slice,
            }) => match syntax {
                HoconSyntaxKind::Equals | HoconSyntaxKind::Colon => {
                    self.builder.token(syntax.into(), slice);
                }
                HoconSyntaxKind::Plus => {
                    self.builder.token(syntax.into(), slice);
                    match self.lexer.next() {
                        Some(Token {
                            token: syntax,
                            slice,
                        }) => match syntax {
                            HoconSyntaxKind::Equals => {
                                self.builder.token(syntax.into(), slice);
                            }
                            _ => {
                                self.builder.token(HoconSyntaxKind::Error.into(), slice);
                            }
                        },
                        None => {
                            //TODO error
                        }
                    }
                }
                HoconSyntaxKind::Ident
                | HoconSyntaxKind::LeftBrace
                | HoconSyntaxKind::LeftBracket => {
                    self.lexer.rewind();
                }
                _ => {
                    self.builder.token(HoconSyntaxKind::Error.into(), slice);
                }
            },
            None => {
                // TODO error
            }
        }
        node_scope!(self.builder, HoconSyntaxKind::Value, self.parse_value());
    }

    fn parse_wihtespace(&mut self) {
        while let Some(Token {
            token: syntax,
            slice,
        }) = self.lexer.next()
        {
            match syntax {
                HoconSyntaxKind::HorizontalWhitespace | HoconSyntaxKind::LineEnding => {
                    self.builder.token(syntax.into(), slice);
                }
                _ => {
                    self.lexer.rewind();
                    break;
                }
            }
        }
    }

    fn parse_array(&mut self) {}
}
