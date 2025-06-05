use std::hash::Hash;

use crate::{FullyQualifiedDomainName, Type};

/// A uniquely identified Record identity.
///
/// Encompasses the tuple of (fqdn, type, rdata), which should uniquely identify
/// it within a zone, since zones cannot contain records which are not unique
/// across these parameters.
///
/// Can be used to store records in HashMaps/HashSets
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordIdent {
    pub fqdn: FullyQualifiedDomainName,
    pub r#type: Type,
    pub rdata: String,
}
