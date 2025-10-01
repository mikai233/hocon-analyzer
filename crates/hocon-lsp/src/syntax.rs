use logos::{Lexer, Logos};

#[derive(Debug, Logos, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u16)]
enum SyntaxKind {
    #[regex(r"[\u0009\u000A\u000B\u000C\u000D\u001C-\u001F\uFEFF\p{Zs}\p{Zl}\p{Zp}]+")]
    Whitespace,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[token(":")]
    Colon,
    #[regex(r"true|false")]
    Bool,
    #[regex(r#"[^$"{}\[\]:=,+#`^?!@*&\\.\u0009\u000A\u000B\u000C\u000D\u001C-\u001F\uFEFF\p{Zs}\p{Zl}\p{Zp}]+"#)]
    UnquotedString,
    #[regex(r#"""#, lex_quoted_string)]
    QuotedString,
    #[regex(r#"""""#, lex_multiline_string)]
    MultiLineString,
    #[token("[")]
    BracketStart,
    #[token("]")]
    BracketEnd,
    #[token("{")]
    BraceStart,
    #[token("}")]
    BraceEnd,
    #[regex(r"#[^\n\r]*")]
    HashComment,
    #[regex(r"//[^\n\r]*")]
    SlashComment,
    #[regex(r#"\$\{\??"#)]
    Substitution,
    #[token(".")]
    Dot,
    #[regex(r"-?(0|[1-9][0-9]*)", priority = 3)]
    Int,
    #[regex(r"-?(0|[1-9][0-9]*)\.[0-9]+([eE][+-]?[0-9]+)?")]
    #[regex(r"-?(0|[1-9][0-9]*)([eE][+-]?[0-9]+)")]
    Float,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(value: SyntaxKind) -> Self {
        Self(value as u16)
    }
}

fn lex_multiline_string(lex: &mut Lexer<SyntaxKind>) -> Option<()> {
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

fn lex_quoted_string(lex: &mut Lexer<SyntaxKind>) -> Option<()> {
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

    use crate::syntax::SyntaxKind;

    #[test]
    fn test_lexer() {
        let mut lex = SyntaxKind::lexer(r#"include required(file("a")) 1.0"#);
        while let Some(kind) = lex.next() {
            let kind = kind.unwrap();
            let slice = lex.slice();
            println!("kind:{:?}", kind);
            println!("slice:{:?}", slice);
        }
    }
}
