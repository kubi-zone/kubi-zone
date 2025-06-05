mod class;
mod dn;
mod fqdn;
mod ident;
mod pattern;
mod pqdn;
mod segment;
mod r#type;

pub use class::Class;
pub use dn::DomainName;
pub use fqdn::FullyQualifiedDomainName;
pub use ident::RecordIdent;
pub use pattern::{Pattern, PatternSegment};
pub use pqdn::PartiallyQualifiedDomainName;
pub use segment::DomainSegment;
pub use r#type::Type;

pub mod error {
    pub use crate::fqdn::FullyQualifiedDomainNameError;
    pub use crate::pattern::PatternSegmentError;
    pub use crate::pqdn::PartiallyQualifiedDomainNameError;
    pub use crate::segment::DomainSegmentError;
}
