mod record;
mod zone;

use std::fmt::Display;

use kubizone_common::FullyQualifiedDomainName;
pub use record::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use zone::*;

/// Reference to a Zone, optionally in a specific namespace.
#[derive(
    Serialize, Deserialize, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ZoneRef {
    pub name: String,
    pub namespace: Option<String>,
}

/// Functionality common between Zones and Records, such as fetching the Fully Qualified Domain Name
/// of the resource, or parsing the parent zone label.
pub trait DomainExt {
    /// Fetch the computed FQDN from this resource, if one has been set.
    fn fqdn(&self) -> Option<&FullyQualifiedDomainName>;

    /// Retrieve the kubi.zone/parent-zone label as a ZoneRef, if present.
    fn parent(&self) -> Option<ZoneRef>;
}

impl ZoneRef {
    /// Serialize the ZoneRef into a label-compatible format.
    pub fn as_label(&self) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}.{namespace}", self.name)
        } else {
            self.name.clone()
        }
    }
}

impl From<&str> for ZoneRef {
    fn from(s: &str) -> Self {
        if let Some((name, namespace)) = s.split_once('.') {
            ZoneRef {
                name: name.to_string(),
                namespace: Some(namespace.to_string()),
            }
        } else {
            // TODO: Might be valuable to do some validation here.
            ZoneRef {
                name: s.to_string(),
                namespace: None,
            }
        }
    }
}

impl From<&String> for ZoneRef {
    fn from(value: &String) -> Self {
        ZoneRef::from(value.as_str())
    }
}

impl From<String> for ZoneRef {
    fn from(value: String) -> Self {
        ZoneRef::from(value.as_str())
    }
}

impl Display for ZoneRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(namespace) = &self.namespace {
            write!(f, "{namespace}/{}", self.name)
        } else {
            f.write_str(&self.name)
        }
    }
}

pub mod defaults {
    use kubizone_common::Class;

    pub const CLASS: Class = Class::IN;
    pub(super) fn class() -> Class {
        CLASS
    }
}
