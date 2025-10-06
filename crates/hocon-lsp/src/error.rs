use std::{fmt::Display, range::Range};

use rowan::{TextRange, TextSize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Error {
    pub(crate) range: TextRange,
    pub(crate) message: String,
}

impl Error {
    pub(crate) fn new(range: TextRange, message: impl Into<String>) -> Self {
        Self {
            range,
            message: message.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at:{:?}", self.message, self.range)
    }
}

impl std::error::Error for Error {}
