use std::collections::BTreeMap;

use kube::{CustomResource, ResourceExt};
use kubizone_crds::v1alpha1::ZoneRef;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A [`ZoneFile`] references an upstream [`Zone`](kubizone_crds::v1alpha1::Zone) and (re)builds
/// a configmap of the same name, whenever the zone changes, automatically incrementing
/// serials as necessary.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Hash)]
// The 'dev' feature flag puts the resource in a separate dev.kubi.zone group,
// instead of the real one. This way you can have the production and dev versions
// of kubizone resources running side by side, without interfering with each other.
#[cfg_attr(
    feature = "dev",
    kube(
        group = "dev.kubi.zone",
        version = "v1alpha1",
        kind = "ZoneFile",
        namespaced
    )
)]
#[cfg_attr(
    not(feature = "dev"),
    kube(
        group = "kubi.zone",
        version = "v1alpha1",
        kind = "ZoneFile",
        namespaced
    )
)]
#[kube(status = "ZoneFileStatus")]
//#[kube(printcolumn = r#"{"name":"zone", "jsonPath": ".spec.zoneRef.name", "type": "string"}"#)]
//#[kube(printcolumn = r#"{"name":"serial", "jsonPath": ".status.serial", "type": "string"}"#)]
//#[kube(printcolumn = r#"{"name":"hash", "jsonPath": ".status.hash", "type": "string"}"#)]
#[serde(rename_all = "camelCase")]
pub struct ZoneFileSpec {
    /// Reference to a [`Zone`](kubizone_crds::v1alpha1::Zone), optionally in a different namespace.
    pub zone_refs: Vec<ZoneRef>,

    #[serde(default)]
    pub config_map_name: Option<String>,
}

impl ZoneFile {
    /// Retrieve the [`ZoneFile`]'s `zoneRef`, but populate the `namespace` variable,
    /// if not specified by the zoneref itself.
    pub fn zone_ref(&self) -> Vec<ZoneRef> {
        self.spec
            .zone_refs
            .iter()
            .map(|zone_ref| ZoneRef {
                name: zone_ref.name.clone(),
                namespace: zone_ref
                    .namespace
                    .as_ref()
                    .or(self.namespace().as_ref())
                    .cloned(),
            })
            .collect()
    }
}

/// Describes the current state of the [`ZoneFile`], tracks state of
/// the upstream [`Zone`](kubizone_crds::v1alpha1::Zone), to determine when the
/// output `ConfigMap` should be re-generated.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ZoneFileStatus {
    /// Last observed hash of the upstream [`Zone`](kubizone_crds::v1alpha1::Zone)
    ///
    /// Used by the zonefile controller to trigger configmap rebuilds
    /// and zone serial rotation.
    pub hash: BTreeMap<String, String>,

    /// Serial of the latest generated zonefile.
    ///
    /// The zonefile controller will automatically increment this value
    /// whenever the zonefile configmap is rebuilt, in accordance with
    /// [RFC 1912](https://datatracker.ietf.org/doc/html/rfc1912#section-2.2)
    pub serial: BTreeMap<String, u32>,
}
