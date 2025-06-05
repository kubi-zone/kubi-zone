use std::{fmt::Display, ops::Add};

use thiserror::Error;

use crate::{DomainName, FullyQualifiedDomainName, PartiallyQualifiedDomainName};

/// Segment of a domain.
///
/// This is the part between dots.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainSegment(String);

impl DomainSegment {
    /// Constructs a new DomainSegment without checking the validity of it.
    pub fn new_unchecked(segment: &str) -> Self {
        DomainSegment(segment.to_string())
    }

    /// Length in characters of the domain segment.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the segment is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // Returns true if the segment is equal to "*"
    pub fn is_wildcard(&self) -> bool {
        self.0 == "*"
    }
}

/// Produced when attempting to construct a [`DomainSegment`] from
/// an invalid string.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DomainSegmentError {
    /// Domain name segments can contain hyphens, but crucially:
    ///
    /// * Not at the beginning of a segment.
    /// * Not at the end of a segment.
    /// * Not at the 3rd and 4th position *simultaneously* (used for [Punycode encoding](https://en.wikipedia.org/wiki/Punycode))
    #[error("illegal hyphen at position {0}")]
    IllegalHyphen(usize),
    /// Segment contains invalid character.
    #[error("invalid character {0}")]
    InvalidCharacter(char),
    /// Domain segment is longer than the permitted 63 characters.
    #[error("segment too long {0} > 63")]
    TooLong(usize),
    /// Domain segment is empty.
    #[error("segment is an empty string")]
    EmptyString,
    /// Domain segments can be wildcards, but must then *only* contain the wildcard.
    #[error("wildcard segments must have length 1")]
    NonStandaloneWildcard,
}

const VALID_CHARACTERS: &str = "_-0123456789abcdefghijklmnopqrstuvwxyz*";

impl TryFrom<&str> for DomainSegment {
    type Error = DomainSegmentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_ascii_lowercase();

        if value.is_empty() {
            return Err(DomainSegmentError::EmptyString);
        }

        if value.len() > 63 {
            return Err(DomainSegmentError::TooLong(value.len()));
        }

        if value.contains('*') && value.len() != 1 {
            return Err(DomainSegmentError::NonStandaloneWildcard);
        }

        if let Some(character) = value.chars().find(|c| !VALID_CHARACTERS.contains(*c)) {
            return Err(DomainSegmentError::InvalidCharacter(character));
        }

        if value.starts_with('-') {
            return Err(DomainSegmentError::IllegalHyphen(1));
        }

        if value.ends_with('-') {
            return Err(DomainSegmentError::IllegalHyphen(value.len()));
        }

        if value.get(2..4) == Some("--") {
            return Err(DomainSegmentError::IllegalHyphen(3));
        }

        Ok(DomainSegment(value))
    }
}

impl TryFrom<String> for DomainSegment {
    type Error = DomainSegmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for DomainSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for DomainSegment {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Add for DomainSegment {
    type Output = PartiallyQualifiedDomainName;

    fn add(self, rhs: Self) -> Self::Output {
        PartiallyQualifiedDomainName::from_iter([self, rhs])
    }
}

impl Add<PartiallyQualifiedDomainName> for DomainSegment {
    type Output = PartiallyQualifiedDomainName;

    fn add(self, mut rhs: PartiallyQualifiedDomainName) -> Self::Output {
        rhs.0.insert(0, self);
        rhs
    }
}

impl Add<&PartiallyQualifiedDomainName> for DomainSegment {
    type Output = PartiallyQualifiedDomainName;

    fn add(self, rhs: &PartiallyQualifiedDomainName) -> Self::Output {
        let mut out = rhs.clone();
        out.0.insert(0, self);
        out
    }
}

impl Add<FullyQualifiedDomainName> for DomainSegment {
    type Output = FullyQualifiedDomainName;

    fn add(self, mut rhs: FullyQualifiedDomainName) -> Self::Output {
        rhs.0.insert(0, self);
        rhs
    }
}

impl Add<&FullyQualifiedDomainName> for DomainSegment {
    type Output = FullyQualifiedDomainName;

    fn add(self, rhs: &FullyQualifiedDomainName) -> Self::Output {
        let mut out = rhs.clone();
        out.0.insert(0, self);
        out
    }
}

impl Add<DomainName> for DomainSegment {
    type Output = DomainName;

    fn add(self, rhs: DomainName) -> Self::Output {
        match rhs {
            DomainName::Full(full) => DomainName::Full(self + full),
            DomainName::Partial(partial) => DomainName::Partial(self + partial),
        }
    }
}

impl Add<&DomainName> for DomainSegment {
    type Output = DomainName;

    fn add(self, rhs: &DomainName) -> Self::Output {
        match rhs {
            DomainName::Full(full) => DomainName::Full(self + full),
            DomainName::Partial(partial) => DomainName::Partial(self + partial),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::segment::{DomainSegment, DomainSegmentError};

    #[test]
    fn segment_construction() {
        assert_eq!(DomainSegment::try_from("abcd").unwrap().as_ref(), "abcd");

        assert_eq!(
            DomainSegment::try_from(""),
            Err(DomainSegmentError::EmptyString)
        );
    }

    #[test]
    fn invalid_character() {
        assert_eq!(
            DomainSegment::try_from("ab.cd"),
            Err(DomainSegmentError::InvalidCharacter('.'))
        );
    }

    #[test]
    fn invalid_hyphens() {
        assert_eq!(DomainSegment::try_from("ab-cd").unwrap().as_ref(), "ab-cd");

        assert_eq!(DomainSegment::try_from("abc-d").unwrap().as_ref(), "abc-d");

        assert_eq!(
            DomainSegment::try_from("ab--cd"),
            Err(DomainSegmentError::IllegalHyphen(3))
        );

        assert_eq!(
            DomainSegment::try_from("-abcd"),
            Err(DomainSegmentError::IllegalHyphen(1))
        );

        assert_eq!(
            DomainSegment::try_from("abcd-"),
            Err(DomainSegmentError::IllegalHyphen(5))
        );
    }

    #[test]
    fn wildcards() {
        assert_eq!(DomainSegment::try_from("*").unwrap().as_ref(), "*");

        assert!(DomainSegment::try_from("*").unwrap().is_wildcard())
    }
}
