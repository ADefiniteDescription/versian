#![feature(let_chains)]
#![feature(associated_type_defaults)]

#[cfg(feature = "cmp")]
use std::cmp::Ordering;
#[cfg(feature = "cmp")]
use std::ffi::CString;
use std::{fmt, str::FromStr};

pub mod error;
pub mod validations;

use crate::error::DebianVersionError;
use crate::validations::ValidateUpstreamVersion;

#[cfg(feature = "cmp")]
use rust_apt::util::cmp_versions;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebianVersion {
    pub epoch: Option<usize>,
    pub upstream_version: String,
    pub debian_revision: Option<String>,
}

macro_rules! bail_empty {
    ($s:expr) => {
        if $s.is_empty() {
            return Err(DebianVersionError::Empty);
        }
    };
}

#[allow(dead_code)]
fn split_upstream_revision(s: &str) -> Result<(&str, Option<&str>)> {
    bail_empty!(s);

    Ok(s.split_once('-')
        .map_or_else(|| (s, None), |(upt, rev)| (upt, Some(rev))))
}

pub type Version<T> = (Option<T>, T, Option<T>);
pub type Result<T> = std::result::Result<T, DebianVersionError>;

#[allow(dead_code)]
#[allow(unused_variables)]
fn parse_version<T: AsRef<str>>(s: T) -> Result<Version<T>> {
    let s = s.as_ref();
    let (epoch, rest) = s.split_once(':').unzip();

    if let Some(rest) = rest {
        bail_empty!(rest);

        let (upstream, revision) = rest
            .split_once('-')
            .unwrap_or_else(|| (rest, Default::default()));
    }

    todo!();

    // todo!()
}

#[cfg(feature = "cmp")]
impl PartialOrd for DebianVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(cmp_versions(&self.to_string(), &other.to_string()))
    }
}

impl DebianVersion {
    /// Returns a formatted `DebianVersion` of the form: [epoch]:[upstream version]-[debian
    /// revision]. Epochs and debian revision substrings are optional.
    #[inline]
    pub fn version(&self) -> String {
        self.to_string()
    }

    /// Returns the epoch of the [`DebianVersion`](), where an epoch is a (usually small) unsized
    /// integer.
    #[inline]
    pub fn epoch(&self) -> Option<usize> {
        self.epoch
    }

    /// Returns a mutable reference to the epoch of the [`DebianVersion`](crate::DebianVersion),
    /// where an epoch is a (usually small) unsized integer.
    #[inline]
    pub fn mut_epoch(&mut self) -> Option<&mut usize> {
        self.epoch.as_mut()
    }

    /// Returns the [`DebianVersion`](crate::DebianVersion) upstream version.
    #[inline]
    pub fn upstream_version(&self) -> &str {
        &self.upstream_version
    }

    /// Returns a mutable reference to the [`DebianVersion`](crate::DebianVersion) upstream version.
    #[inline]
    pub fn mut_upstream_version(&mut self) -> &mut str {
        &mut self.upstream_version
    }

    /// Applies the argument `f` to the [`DebianVersion`](crate::DebianVersion) `upstream_version`
    /// field and returns a mutable reference to that field.
    #[inline]
    pub fn map_upstream_version_with<F>(&mut self, mut f: F) -> &mut str
    where
        F: FnMut(&mut str) -> &mut str,
    {
        f(self.mut_upstream_version());
        &mut self.upstream_version
    }

    /// Returns a shared reference to the [`DebianVersion`](crate::DebianVersion) Debian revision.
    #[inline]
    pub fn debian_revision(&self) -> &Option<String> {
        &self.debian_revision
    }

    /// Returns a mutable reference to the [`DebianVersion`](crate::DebianVersion) Debian revision.
    #[inline]
    pub fn mut_debian_revision(&mut self) -> &mut Option<String> {
        &mut self.debian_revision
    }

    /// Applies the argument `f` to the [`DebianVersion`](crate::DebianVersion) `debian_revision`
    /// field.
    #[inline]
    pub fn map_debian_revision_with<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut str) -> &mut str,
    {
        self.debian_revision = self
            .debian_revision
            .as_mut()
            .map(|x| f(x))
            .map(|x| x.to_owned());
    }
}

impl fmt::Display for DebianVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Epochs are optional and separated from the rest by ':', therefore we need to handle this
        // case separately.
        if let Some(epoch) = self.epoch {
            // `DebianVersion` may or may not contain a Debian revision.
            if let Some(ref revision) = self.debian_revision {
                write!(f, "{}:{}-{}", epoch, self.upstream_version, revision)
            } else {
                write!(f, "{}:{}", epoch, self.upstream_version)
            }
        } else {
            // `DebianVersion` may or may not contain a Debian revision.
            if let Some(ref revision) = self.debian_revision {
                write!(f, "{}-{}", self.upstream_version, revision)
            } else {
                write!(f, "{}", self.upstream_version)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Epoch(pub usize);

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Epoch {
    type Err = DebianVersionError;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Err(DebianVersionError::InvalidEpoch);
        }

        match s.split_once(':') {
            Some((epoch, _)) => Ok(Self(epoch.parse::<usize>()?)),
            None => Err(DebianVersionError::InvalidEpoch),
        }
    }
}

pub enum VersionKinds {
    /// Epoch and upstream version.
    EpochUp,
    /// Epoch, upstream, and revision.
    EpUpRe,
    /// Only upstream.
    U,
    ///
    UR,
}

impl FromStr for DebianVersion {
    type Err = DebianVersionError;

    fn from_str(value: &str) -> Result<Self> {
        // A [`DebianVersion`] must never be empty.
        if value.is_empty() {
            return Err(DebianVersionError::Empty);
        }

        let mut epoch = None;

        // The Debian version string contains an epoch.
        match value.split_once(':') {
            Some((first, rest)) => {
                epoch = match first.parse::<usize>() {
                    Ok(inner) => Some(inner),
                    Err(_) => return Err(DebianVersionError::InvalidEpoch),
                };

                if let Some((upstream_version, debian_revision)) = rest.rsplit_once('-') {
                    if upstream_version.validate_with_revision()? {
                        return Ok(Self {
                            epoch,
                            upstream_version: upstream_version.to_string(),
                            debian_revision: Some(debian_revision.to_string()),
                        });
                    }
                } else {
                    if rest.validate_without_revision()? {
                        return Ok(Self {
                            epoch,
                            upstream_version: rest.to_string(),
                            debian_revision: None,
                        });
                    }
                }
            }
            None => {
                if let Some((upstream_version, debian_revision)) = value.rsplit_once('-') {
                    if upstream_version.validate_with_revision()? {
                        return Ok(Self {
                            epoch,
                            upstream_version: upstream_version.to_string(),
                            debian_revision: Some(debian_revision.to_string()),
                        });
                    }
                } else {
                    if value.validate_without_revision()? {
                        return Ok(Self {
                            epoch,
                            upstream_version: value.to_string(),
                            debian_revision: None,
                        });
                    }
                }
            }
        }

        Ok(Self {
            epoch,
            upstream_version: value.to_string(),
            debian_revision: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use more_asserts as ma;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let parsed = "".parse::<DebianVersion>();
        let expected_error = DebianVersionError::Empty;
        let actual_error = parsed.unwrap_err();

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn only_dashes() {
        let parsed = "---".parse::<DebianVersion>();
        let expected_error = DebianVersionError::UpstreamStartWithDigit;
        let actual_error = parsed.unwrap_err();

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn valid_version() {
        let version = "5.10.104-tegra-35.2.1-20230124153320";
        let parsed = version.parse::<DebianVersion>();

        assert_eq!(
            parsed,
            Ok(DebianVersion {
                epoch: None,
                upstream_version: String::from("5.10.104-tegra-35.2.1"),
                debian_revision: Some("20230124153320".to_string()),
            }),
        )
    }

    #[cfg(feature = "cmp")]
    #[test]
    fn cmp_versions() {
        let parsed1 = DebianVersion::from_str("5.10.104-tegra-35.2.1-20230124153320");
        let parsed2 = DebianVersion::from_str("5.10.104-tegra-35.3.1-20230124153320");
        assert!(parsed1.is_ok());
        assert!(parsed2.is_ok());
        ma::assert_lt!(parsed1.unwrap(), parsed2.unwrap());
    }
}
