use std::{
    borrow::Cow,
    fmt::{Display, Write},
    ops::Add,
};

use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{de::Error, Deserialize, Serialize};
use thiserror::Error;

use crate::{
    segment::{DomainSegment, DomainSegmentError},
    FullyQualifiedDomainName,
};

/// Produced when attempting to construct a [`PartiallyQualifiedDomainName`]
/// from an invalid string.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartiallyQualifiedDomainNameError {
    /// The parsed string is not partially qualified. That is, it contains
    /// a trailing dot making it fully qualified.
    #[error("domain is fully qualified")]
    DomainIsFullyQualified,
    /// One or more of the segments of the domain specified in the string
    /// are invalid.
    #[error("{0}")]
    SegmentError(#[from] DomainSegmentError),
    /// Wildcard segments must only appear at the beginning of a record.
    #[error("non-leading wildcard segment")]
    NonLeadingWildcard,
}

/// Partially qualified domain name (PQDN).
///
/// A partially qualified domain name is an incomplete domain, meaning
/// the domain name is (potentially) a subdomain of another unknown domain.
/// Unlike fully qualified domain names, PQDNs indicate only some of the
/// path within the domain name system.
///
/// Partially qualified domain names are often used when the root of the
/// domain name is not known, or specified elsewhere.
///
/// See also [`FullyQualifiedDomainName`]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartiallyQualifiedDomainName(pub(crate) Vec<DomainSegment>);

impl PartiallyQualifiedDomainName {
    /// Appends the fqdn to the end of the partial domain.
    pub fn with_origin(&self, origin: &FullyQualifiedDomainName) -> FullyQualifiedDomainName {
        self + origin
    }

    /// Iterates over all [`DomainSegment`]s that make up the domain name.
    pub fn iter(&self) -> core::slice::Iter<'_, DomainSegment> {
        self.0.iter()
    }

    /// Length of the fully qualified domain name as a string.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.iter().map(|segment| segment.len()).sum::<usize>() + self.0.len()
    }

    /// Coerce the domain name into a fully qualified one.
    pub fn into_fully_qualified(self) -> FullyQualifiedDomainName {
        FullyQualifiedDomainName(self.0)
    }

    /// Coerce the domain name into a fully qualified one.
    pub fn to_fully_qualified(&self) -> FullyQualifiedDomainName {
        FullyQualifiedDomainName(self.0.clone())
    }
}

impl FromIterator<DomainSegment> for PartiallyQualifiedDomainName {
    fn from_iter<T: IntoIterator<Item = DomainSegment>>(iter: T) -> Self {
        PartiallyQualifiedDomainName(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a DomainSegment> for PartiallyQualifiedDomainName {
    fn from_iter<T: IntoIterator<Item = &'a DomainSegment>>(iter: T) -> Self {
        PartiallyQualifiedDomainName(iter.into_iter().cloned().collect())
    }
}

impl TryFrom<String> for PartiallyQualifiedDomainName {
    type Error = PartiallyQualifiedDomainNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for PartiallyQualifiedDomainName {
    type Error = PartiallyQualifiedDomainNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with('.') {
            Err(PartiallyQualifiedDomainNameError::DomainIsFullyQualified)
        } else {
            let segments: Vec<DomainSegment> =
                Result::from_iter(value.split('.').map(DomainSegment::try_from))?;

            if segments.iter().skip(1).any(DomainSegment::is_wildcard) {
                return Err(PartiallyQualifiedDomainNameError::NonLeadingWildcard);
            }

            Ok(PartiallyQualifiedDomainName(segments))
        }
    }
}

impl Display for PartiallyQualifiedDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, segment) in self.0.iter().enumerate() {
            segment.fmt(f)?;
            if index != self.0.len() - 1 {
                f.write_char('.')?;
            }
        }

        Ok(())
    }
}

impl Add<&FullyQualifiedDomainName> for &PartiallyQualifiedDomainName {
    type Output = FullyQualifiedDomainName;

    fn add(self, rhs: &FullyQualifiedDomainName) -> Self::Output {
        FullyQualifiedDomainName::from_iter(self.0.iter().chain(rhs.iter()).cloned())
    }
}

impl Add for &PartiallyQualifiedDomainName {
    type Output = PartiallyQualifiedDomainName;

    fn add(self, rhs: &PartiallyQualifiedDomainName) -> Self::Output {
        PartiallyQualifiedDomainName::from_iter(self.0.iter().chain(rhs.iter()).cloned())
    }
}

impl AsRef<[DomainSegment]> for PartiallyQualifiedDomainName {
    fn as_ref(&self) -> &[DomainSegment] {
        self.0.as_ref()
    }
}

impl PartialEq<String> for PartiallyQualifiedDomainName {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<str> for PartiallyQualifiedDomainName {
    fn eq(&self, other: &str) -> bool {
        self.to_string().eq(other)
    }
}

impl JsonSchema for PartiallyQualifiedDomainName {
    fn schema_name() -> Cow<'static, str> {
        <String as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        <String as schemars::JsonSchema>::json_schema(gen)
    }
}

impl<'de> Deserialize<'de> for PartiallyQualifiedDomainName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::try_from(value).map_err(D::Error::custom)
    }
}

impl Serialize for PartiallyQualifiedDomainName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::PartiallyQualifiedDomainNameError, segment::DomainSegment, FullyQualifiedDomainName,
        PartiallyQualifiedDomainName,
    };

    #[test]
    fn construct_pqdn() {
        assert_eq!(
            PartiallyQualifiedDomainName::try_from("example.org").unwrap(),
            PartiallyQualifiedDomainName::from_iter([
                DomainSegment::try_from("example").unwrap(),
                DomainSegment::try_from("org").unwrap()
            ])
        );
    }

    #[test]
    fn pqdn_from_fqdn_fails() {
        assert_eq!(
            PartiallyQualifiedDomainName::try_from("example.org."),
            Err(PartiallyQualifiedDomainNameError::DomainIsFullyQualified)
        );
    }

    #[test]
    fn addition() {
        assert_eq!(
            &PartiallyQualifiedDomainName::try_from("test").unwrap()
                + &FullyQualifiedDomainName::try_from("example.org.").unwrap(),
            FullyQualifiedDomainName::try_from("test.example.org.").unwrap()
        )
    }

    #[test]
    fn pqdn_addition() {
        assert_eq!(
            &PartiallyQualifiedDomainName::try_from("test").unwrap()
                + &PartiallyQualifiedDomainName::try_from("example").unwrap(),
            PartiallyQualifiedDomainName::try_from("test.example").unwrap()
        )
    }
}
