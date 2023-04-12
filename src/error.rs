use std::{error, fmt, num::ParseIntError};

#[derive(Debug, PartialEq)]
pub enum DebianVersionError {
    InvalidEpoch,
    Empty,
    InvalidUpstream,
    EmptyUpstream,
    UpstreamStartWithDigit,
    UpstreamInvalidCharacters,
    EmptyRevision,
    RevisionInvalidCharacters,
    InvalidFlags,
}

impl From<ParseIntError> for DebianVersionError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidEpoch
    }
}

impl fmt::Display for DebianVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DebianVersionError::InvalidEpoch => write!(f, "Epochs must be numeric."),
            DebianVersionError::Empty => write!(f, "Version is empty."),
            DebianVersionError::InvalidUpstream => write!(f, "Invalid upstream version."),
            DebianVersionError::EmptyUpstream => write!(f, "Upstream version is empty."),
            DebianVersionError::UpstreamStartWithDigit => {
                write!(f, "Upstream version must start with a digit.")
            }
            DebianVersionError::UpstreamInvalidCharacters => {
                write!(f, "Upstream version contains invalid characters.")
            }
            DebianVersionError::EmptyRevision => write!(f, "Debian revision is empty."),
            DebianVersionError::RevisionInvalidCharacters => {
                write!(f, "Debian revision contains invalid characters.")
            }
            DebianVersionError::InvalidFlags => write!(f, "Invalid flag combination."),
        }
    }
}

impl error::Error for DebianVersionError {}
