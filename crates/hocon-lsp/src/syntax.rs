use logos::{Lexer, Logos};

#[derive(Debug, Logos, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u16)]
pub(crate) enum HoconSyntaxKind {
    //Tokens
    #[regex(r"[\u0009\u000B\u000C\u000D\u001C-\u001F\uFEFF\p{Zs}\p{Zl}\p{Zp}]+")]
    HorizontalWhitespace,
    #[regex(r"(\n|\r\n)")]
    LineEnding,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[token("+")]
    Plus,
    #[token(":")]
    Colon,
    #[regex(r#"[^$"{}\[\]:=,+#`^?!@*&\\.\u0009\u000A\u000B\u000C\u000D\u001C-\u001F\uFEFF\p{Zs}\p{Zl}\p{Zp}]+"#)]
    #[regex(r#"""#, lex_quoted_string)]
    #[regex(r#"""""#, lex_multiline_string)]
    Ident,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[regex(r"#[^\n\r]*")]
    HashComment,
    #[regex(r"//[^\n\r]*")]
    SlashComment,
    #[token("$")]
    Dollar,
    #[token("?")]
    QuestionMark,
    #[token(".")]
    Dot,
    Error,
    Bool,
    Number,
    UnquotedString,
    QuotedString,
    MultilineString,
    LeftParen,
    RightParen,
    Include,
    Required,
    File,
    Url,
    Classpath,
    //Nodes
    Root,
    Entry,
    Key,
    Value,
    Object,
    Array,
    Substitution,
    PathExpression,
    Inclusion,
}

impl From<HoconSyntaxKind> for rowan::SyntaxKind {
    fn from(value: HoconSyntaxKind) -> Self {
        Self(value as u16)
    }
}

fn lex_multiline_string(lex: &mut Lexer<HoconSyntaxKind>) -> Option<()> {
    let rest = &lex.remainder();

    let mut end = 0;
    while end + 3 <= rest.len() {
        if &rest[end..end + 3] == "\"\"\"" {
            lex.bump(end + 3);
            return Some(());
        } else {
            end += 1;
        }
    }
    None
}

fn lex_quoted_string(lex: &mut Lexer<HoconSyntaxKind>) -> Option<()> {
    let remainder: &str = lex.remainder();
    let mut escaped = false;

    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = !escaped;
            continue;
        }

        if c == '"' && !escaped {
            lex.bump(total_len);
            return Some(());
        }

        escaped = false;
    }
    None
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::syntax::HoconSyntaxKind;

    #[test]
    fn test_lexer() {
        let mut lex = HoconSyntaxKind::lexer(r#"include required(file("a")) 1.0"#);
        while let Some(kind) = lex.next() {
            let kind = kind.unwrap();
            let slice = lex.slice();
            println!("kind:{:?}", kind);
            println!("slice:{:?}", slice);
        }
    }

    #[test]
    fn test_lexer2() {
        let config = std::fs::read_to_string(
            "/Users/mikai/IdeaProjects/akka/akka-actor/src/main/resources/reference.conf",
        )
        .unwrap();
        let mut lex = HoconSyntaxKind::lexer(&config);
        while let Some(kind) = lex.next() {
            let kind = kind.unwrap();
            let slice = lex.slice();
            println!("kind:{:?}", kind);
            println!("slice:{:?}", slice);
        }
    }
}
