pub mod v1alpha1;

#[cfg(feature = "dev")]
pub const TARGET_ZONEFILE_LABEL: &str = "dev.ubi.zone/zonefile";
#[cfg(not(feature = "dev"))]
pub const TARGET_ZONEFILE_LABEL: &str = "kubi.zone/zonefile";
