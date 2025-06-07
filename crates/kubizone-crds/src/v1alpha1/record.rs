use std::fmt::Display;

use kube::{CustomResource, Resource, ResourceExt};
use kubizone_common::{Class, DomainName, FullyQualifiedDomainName, RecordIdent, Type};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::PARENT_ZONE_LABEL;

use super::{DomainExt, ZoneRef};

#[derive(
    CustomResource,
    Deserialize,
    Serialize,
    Clone,
    Debug,
    Default,
    JsonSchema,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
// The 'dev' feature flag puts the resource in a separate dev.kubi.zone group,
// instead of the real one. This way you can have the production and dev versions
// of kubizone resources running side by side, without interfering with each other.
#[cfg_attr(
    feature = "dev",
    kube(
        group = "dev.kubi.zone",
        version = "v1alpha1",
        kind = "Record",
        namespaced
    )
)]
#[cfg_attr(
    not(feature = "dev"),
    kube(group = "kubi.zone", version = "v1alpha1", kind = "Record", namespaced)
)]
#[kube(status = "RecordStatus")]
#[kube(printcolumn = r#"{"name":"domain name", "jsonPath": ".spec.domainName", "type": "string"}"#)]
#[kube(printcolumn = r#"{"name":"class", "jsonPath": ".spec.class", "type": "string"}"#)]
#[kube(printcolumn = r#"{"name":"type", "jsonPath": ".spec.type", "type": "string"}"#)]
#[kube(printcolumn = r#"{"name":"data", "jsonPath": ".spec.rdata", "type": "string"}"#)]
#[kube(printcolumn = r#"{"name":"fqdn", "jsonPath": ".status.fqdn", "type": "string"}"#)]
#[kube(
    printcolumn = r#"{"name":"parent", "jsonPath": ".metadata.labels.kubi\\.zone/parent-zone", "type": "string"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct RecordSpec {
    pub domain_name: DomainName,
    pub zone_ref: Option<ZoneRef>,
    #[serde(rename = "type")]
    pub type_: Type,
    #[serde(default = "super::defaults::class")]
    pub class: Class,
    pub ttl: Option<u32>,
    pub rdata: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct RecordStatus {
    pub fqdn: Option<FullyQualifiedDomainName>,
}

impl DomainExt for Record {
    fn fqdn(&self) -> Option<&FullyQualifiedDomainName> {
        self.status.as_ref().and_then(|status| status.fqdn.as_ref())
    }

    fn parent(&self) -> Option<ZoneRef> {
        self.meta()
            .labels
            .as_ref()?
            .get(PARENT_ZONE_LABEL)
            .map(ZoneRef::from)
    }
}

impl RecordSpec {
    pub fn is_internet(&self) -> bool {
        self.class == Class::IN
    }

    pub fn is_chaos(&self) -> bool {
        self.class == Class::CH
    }

    pub fn is_hesiod(&self) -> bool {
        self.class == Class::HS
    }

    pub fn is_a(&self) -> bool {
        self.type_ == Type::A
    }

    pub fn is_aaaa(&self) -> bool {
        self.type_ == Type::AAAA
    }

    pub fn is_afsdb(&self) -> bool {
        self.type_ == Type::AFSDB
    }

    pub fn is_apl(&self) -> bool {
        self.type_ == Type::APL
    }

    pub fn is_caa(&self) -> bool {
        self.type_ == Type::CAA
    }

    pub fn is_cdnskey(&self) -> bool {
        self.type_ == Type::CDNSKEY
    }

    pub fn is_cds(&self) -> bool {
        self.type_ == Type::CDS
    }

    pub fn is_cert(&self) -> bool {
        self.type_ == Type::CERT
    }

    pub fn is_cname(&self) -> bool {
        self.type_ == Type::CNAME
    }

    pub fn is_csync(&self) -> bool {
        self.type_ == Type::CSYNC
    }

    pub fn is_dhcid(&self) -> bool {
        self.type_ == Type::DHCID
    }

    pub fn is_dlv(&self) -> bool {
        self.type_ == Type::DLV
    }

    pub fn is_dname(&self) -> bool {
        self.type_ == Type::DNAME
    }

    pub fn is_dnskey(&self) -> bool {
        self.type_ == Type::DNSKEY
    }

    pub fn is_ds(&self) -> bool {
        self.type_ == Type::DS
    }

    pub fn is_eui48(&self) -> bool {
        self.type_ == Type::EUI48
    }

    pub fn is_eui64(&self) -> bool {
        self.type_ == Type::EUI64
    }

    pub fn is_hinfo(&self) -> bool {
        self.type_ == Type::HINFO
    }

    pub fn is_hip(&self) -> bool {
        self.type_ == Type::HIP
    }

    pub fn is_https(&self) -> bool {
        self.type_ == Type::HTTPS
    }

    pub fn is_ipseckey(&self) -> bool {
        self.type_ == Type::IPSECKEY
    }

    pub fn is_key(&self) -> bool {
        self.type_ == Type::KEY
    }

    pub fn is_kx(&self) -> bool {
        self.type_ == Type::KX
    }

    pub fn is_loc(&self) -> bool {
        self.type_ == Type::LOC
    }

    pub fn is_mx(&self) -> bool {
        self.type_ == Type::MX
    }

    pub fn is_naptr(&self) -> bool {
        self.type_ == Type::NAPTR
    }

    pub fn is_ns(&self) -> bool {
        self.type_ == Type::NS
    }

    pub fn is_nsec(&self) -> bool {
        self.type_ == Type::NSEC
    }

    pub fn is_nsec3(&self) -> bool {
        self.type_ == Type::NSEC3
    }

    pub fn is_nsec3param(&self) -> bool {
        self.type_ == Type::NSEC3PARAM
    }

    pub fn is_openpgpkey(&self) -> bool {
        self.type_ == Type::OPENPGPKEY
    }

    pub fn is_ptr(&self) -> bool {
        self.type_ == Type::PTR
    }

    pub fn is_rrsig(&self) -> bool {
        self.type_ == Type::RRSIG
    }

    pub fn is_rp(&self) -> bool {
        self.type_ == Type::RP
    }

    pub fn is_sig(&self) -> bool {
        self.type_ == Type::SIG
    }

    pub fn is_smimea(&self) -> bool {
        self.type_ == Type::SMIMEA
    }

    pub fn is_soa(&self) -> bool {
        self.type_ == Type::SOA
    }

    pub fn is_srv(&self) -> bool {
        self.type_ == Type::SRV
    }

    pub fn is_sshfp(&self) -> bool {
        self.type_ == Type::SSHFP
    }

    pub fn is_svcb(&self) -> bool {
        self.type_ == Type::SVCB
    }

    pub fn is_ta(&self) -> bool {
        self.type_ == Type::TA
    }

    pub fn is_tkey(&self) -> bool {
        self.type_ == Type::TKEY
    }

    pub fn is_tlsa(&self) -> bool {
        self.type_ == Type::TLSA
    }

    pub fn is_tsig(&self) -> bool {
        self.type_ == Type::TSIG
    }

    pub fn is_txt(&self) -> bool {
        self.type_ == Type::TXT
    }

    pub fn is_uri(&self) -> bool {
        self.type_ == Type::URI
    }

    pub fn is_zonemd(&self) -> bool {
        self.type_ == Type::ZONEMD
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Unwrap safety: Records are namespaced and therefore always have a name.
        write!(
            f,
            "{}/{}",
            self.metadata.namespace.as_ref().unwrap(),
            self.name_any()
        )
    }
}

impl TryFrom<&Record> for RecordIdent {
    type Error = &'static str;

    fn try_from(value: &Record) -> Result<Self, Self::Error> {
        let fqdn = value
            .status
            .as_ref()
            .ok_or("record's status section is not yet populated")?
            .fqdn
            .as_ref()
            .ok_or("record does not yet have a fully qualified domain name")?
            .clone();

        Ok(RecordIdent {
            fqdn: fqdn.clone(),
            r#type: value.spec.type_,
            rdata: value.spec.rdata.clone(),
        })
    }
}
