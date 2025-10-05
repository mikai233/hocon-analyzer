use std::fmt::Display;

use rowan::TextRange;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Error {
    pub(crate) range: TextRange,
    pub(crate) message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at:{:?}", self.message, self.range)
    }
}

impl std::error::Error for Error {}
