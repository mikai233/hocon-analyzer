use rowan::Language;

use crate::syntax::HoconSyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HoconLanguage;

impl Language for HoconLanguage {
    type Kind = HoconSyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute::<u16, HoconSyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub(crate) type SyntaxNode = rowan::SyntaxNode<HoconLanguage>;
pub(crate) type SyntaxToken = rowan::SyntaxToken<HoconLanguage>;
pub(crate) type SyntaxElement = rowan::SyntaxElement<HoconLanguage>;
