use std::{
    fmt::{Debug, Display, Write},
    ops::Sub,
};

use schemars::{JsonSchema, SchemaGenerator, schema::Schema};
use serde::{Deserialize, Serialize, de::Error};
use thiserror::Error;

use crate::{
    PartiallyQualifiedDomainName,
    segment::{DomainSegment, DomainSegmentError},
};

/// Produced when attempting to construct a [`FullyQualifiedDomainName`]
/// from an invalid string.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FullyQualifiedDomainNameError {
    /// The parsed string is not fully qualified. i.e. it does not contain
    /// a trailing dot.
    #[error("domain is partially qualified")]
    DomainIsPartiallyQualified,
    /// One or more of the segments of the domain specified in the string
    /// are invalid.
    #[error("{0}")]
    SegmentError(#[from] DomainSegmentError),
    /// Wildcard segments must only appear at the beginning of a record.
    #[error("non-leading wildcard segment")]
    NonLeadingWildcard,
}

/// Fully qualified domain name (FQDN).
///
/// A fully qualified domain name is a domain name consisting of
/// a series of [`DomainSegment`]s, and ending in a trailing dot.
/// The trailing dot indicates that this is the entirety of the
/// domain name, and therefore denotes the exact location of the
/// domain within the domain name system.
///
/// See also [`PartiallyQualifiedDomainName`](crate::PartiallyQualifiedDomainName).
#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FullyQualifiedDomainName(pub(crate) Vec<DomainSegment>);

impl FullyQualifiedDomainName {
    /// Iterates over all [`DomainSegment`]s that make up the domain name.
    pub fn iter(&self) -> core::slice::Iter<'_, DomainSegment> {
        self.0.iter()
    }

    /// Returns true if `parent` matches the tail end of `self`.
    pub fn is_subdomain_of(&self, parent: &FullyQualifiedDomainName) -> bool {
        self.0.ends_with(parent.as_ref()) && self != parent
    }

    /// Length of the fully qualified domain name as a string, *including* the trailing dot.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.iter().map(|segment| segment.len()).sum::<usize>() + self.0.len()
    }

    /// Coerce the domain name into a partially qualified one.
    pub fn into_partially_qualified(self) -> PartiallyQualifiedDomainName {
        PartiallyQualifiedDomainName(self.0)
    }

    /// Coerce the domain name into a partially qualified one.
    pub fn to_partially_qualified(&self) -> PartiallyQualifiedDomainName {
        PartiallyQualifiedDomainName(self.0.clone())
    }
}

impl FromIterator<DomainSegment> for FullyQualifiedDomainName {
    fn from_iter<T: IntoIterator<Item = DomainSegment>>(iter: T) -> Self {
        FullyQualifiedDomainName(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a DomainSegment> for FullyQualifiedDomainName {
    fn from_iter<T: IntoIterator<Item = &'a DomainSegment>>(iter: T) -> Self {
        FullyQualifiedDomainName(iter.into_iter().cloned().collect())
    }
}

impl TryFrom<String> for FullyQualifiedDomainName {
    type Error = FullyQualifiedDomainNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for FullyQualifiedDomainName {
    type Error = FullyQualifiedDomainNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.ends_with('.') {
            Err(FullyQualifiedDomainNameError::DomainIsPartiallyQualified)
        } else {
            let segments: Vec<DomainSegment> = Result::from_iter(
                value
                    .trim_end_matches('.')
                    .split('.')
                    .map(DomainSegment::try_from),
            )?;

            if segments.iter().skip(1).any(DomainSegment::is_wildcard) {
                return Err(FullyQualifiedDomainNameError::NonLeadingWildcard);
            }

            Ok(FullyQualifiedDomainName(segments))
        }
    }
}

impl Display for FullyQualifiedDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in &self.0 {
            write!(f, "{}", segment)?;
            f.write_char('.')?;
        }

        Ok(())
    }
}

impl AsRef<[DomainSegment]> for FullyQualifiedDomainName {
    fn as_ref(&self) -> &[DomainSegment] {
        self.0.as_ref()
    }
}

impl PartialEq<String> for FullyQualifiedDomainName {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<str> for FullyQualifiedDomainName {
    fn eq(&self, other: &str) -> bool {
        self.to_string().eq(other)
    }
}

impl JsonSchema for FullyQualifiedDomainName {
    fn schema_name() -> String {
        <String as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        <String as schemars::JsonSchema>::json_schema(generator)
    }
}

impl<'de> Deserialize<'de> for FullyQualifiedDomainName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::try_from(value).map_err(D::Error::custom)
    }
}

impl Serialize for FullyQualifiedDomainName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'a> Sub for &'a FullyQualifiedDomainName {
    type Output = Result<PartiallyQualifiedDomainName, &'a FullyQualifiedDomainName>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut own_segments = self.0.clone().into_iter().rev();
        let parent_segments = rhs.0.iter().rev();

        for parent_domain in parent_segments {
            if !own_segments
                .next()
                .is_some_and(|segment| &segment == parent_domain)
            {
                return Err(self);
            }
        }

        Ok(PartiallyQualifiedDomainName::from_iter(own_segments.rev()))
    }
}

impl Sub for FullyQualifiedDomainName {
    type Output = Result<PartiallyQualifiedDomainName, FullyQualifiedDomainName>;

    fn sub(self, rhs: Self) -> Self::Output {
        match &self - &rhs {
            Ok(partial) => Ok(partial),
            Err(_) => Err(self),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        FullyQualifiedDomainName, PartiallyQualifiedDomainName,
        fqdn::FullyQualifiedDomainNameError, segment::DomainSegment,
    };

    #[test]
    fn construct_fqdn() {
        assert_eq!(
            FullyQualifiedDomainName::try_from("example.org."),
            Ok(FullyQualifiedDomainName::from_iter([
                DomainSegment::try_from("example").unwrap(),
                DomainSegment::try_from("org").unwrap()
            ]))
        );
    }

    #[test]
    fn fqdn_from_pqdn_fails() {
        assert_eq!(
            FullyQualifiedDomainName::try_from("example.org"),
            Err(FullyQualifiedDomainNameError::DomainIsPartiallyQualified)
        );
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            FullyQualifiedDomainName::try_from("www.example.org.").unwrap()
                - FullyQualifiedDomainName::try_from("example.org.").unwrap(),
            Ok(PartiallyQualifiedDomainName::try_from("www").unwrap())
        );

        assert_eq!(
            FullyQualifiedDomainName::try_from("www.example.org.").unwrap()
                - FullyQualifiedDomainName::try_from("org.").unwrap(),
            Ok(PartiallyQualifiedDomainName::try_from("www.example").unwrap())
        );

        assert_eq!(
            FullyQualifiedDomainName::try_from("www.example.org.").unwrap()
                - FullyQualifiedDomainName::try_from("test.org.").unwrap(),
            Err(FullyQualifiedDomainName::try_from("www.example.org.").unwrap())
        );
    }
}
