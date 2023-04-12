use crate::error::DebianVersionError;

pub trait ValidateUpstreamVersion {
    fn validate_with_revision(&self) -> Result<bool, DebianVersionError>;
    fn validate_without_revision(&self) -> Result<bool, DebianVersionError>;
}

impl<T> ValidateUpstreamVersion for T
where
    T: AsRef<str>,
{
    fn validate_with_revision(&self) -> Result<bool, DebianVersionError> {
        let s = self.as_ref();

        if s.is_empty() {
            return Err(DebianVersionError::EmptyUpstream);
        }

        if !s.starts_with(|c| char::is_ascii_digit(&c)) {
            return Err(DebianVersionError::UpstreamStartWithDigit);
        }

        if s.chars().all(|c| {
            char::is_ascii_alphanumeric(&c)
                || ['.', '+', '-', ':', '~'].iter().any(|valid| *valid == c)
        }) {
            return Ok(true);
        } else {
            Err(DebianVersionError::UpstreamInvalidCharacters)
        }
    }

    fn validate_without_revision(&self) -> Result<bool, DebianVersionError> {
        let s = self.as_ref();

        if s.is_empty() {
            return Err(DebianVersionError::EmptyUpstream);
        }

        if !s.starts_with(|c| char::is_ascii_digit(&c)) {
            return Err(DebianVersionError::UpstreamStartWithDigit);
        }

        if only_valid_chars(s) {
            return Ok(true);
        } else {
            Err(DebianVersionError::UpstreamInvalidCharacters)
        }
    }
}

pub trait ValidateDebianRevision {
    fn validate(&self) -> Result<bool, DebianVersionError>;
}

impl<T> ValidateDebianRevision for T
where
    T: AsRef<str>,
{
    fn validate(&self) -> Result<bool, DebianVersionError> {
        let s = self.as_ref();
        if s.is_empty() {
            return Err(DebianVersionError::EmptyRevision);
        }
        if !only_valid_chars(s) {
            return Err(DebianVersionError::RevisionInvalidCharacters);
        }

        Ok(true)
    }
}

pub(crate) fn only_valid_chars(s: &str) -> bool {
    s.chars()
        .all(|c| char::is_ascii_alphanumeric(&c) || c == '+' || c == '.' || c == '~' || c == ':')
}
