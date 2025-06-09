pub mod v1alpha1;

#[cfg(feature = "dev")]
pub const PARENT_ZONE_LABEL: &str = "dev.kubi.zone/parent-zone";
#[cfg(not(feature = "dev"))]
pub const PARENT_ZONE_LABEL: &str = "kubi.zone/parent-zone";
