use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Domain Name System class.
#[derive(
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum Class {
    /// Internet
    #[default]
    #[serde(rename = "IN")]
    IN,
    /// Chaos
    #[serde(rename = "CH")]
    CH,
    /// Hesiod
    #[serde(rename = "HS")]
    HS,
}

impl Class {
    ///  Returns true if `self` is [`Class::IN`]
    pub fn is_internet(&self) -> bool {
        *self == Class::IN
    }

    ///  Returns true if `self` is [`Class::CH`]
    pub fn is_chaos(&self) -> bool {
        *self == Class::CH
    }

    ///  Returns true if `self` is [`Class::HS`]
    pub fn is_hesiod(&self) -> bool {
        *self == Class::HS
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::IN => f.write_str("IN"),
            Class::CH => f.write_str("CH"),
            Class::HS => f.write_str("HS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::Class;

    #[test]
    fn deser() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct ExampleStruct {
            class: Class,
        }

        let example = vec![ExampleStruct { class: Class::IN }];

        let serialized = serde_yaml::to_string(&example).unwrap();
        println!("{serialized}");

        assert_eq!(
            serde_yaml::from_str::<Vec<ExampleStruct>>(&serialized).unwrap(),
            example
        );
    }
}
