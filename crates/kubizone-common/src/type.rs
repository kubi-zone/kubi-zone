use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Domain Name System type.
#[allow(clippy::upper_case_acronyms)]
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
pub enum Type {
    /// Address record
    ///
    /// Returns a 32-bit IPv4 address, most commonly used to map hostnames to an IP address of the host, but it is also used for DNSBLs, storing subnet masks in RFC 1101, etc.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    #[default]
    A,
    /// IPv6 address record
    ///
    /// Returns a 128-bit IPv6 address, most commonly used to map hostnames to an IP address of the host.
    ///
    /// [3596](https://datatracker.ietf.org/doc/html/rfc3596)
    AAAA,
    /// AFS database record
    ///
    /// Location of database servers of an AFS cell. This record is commonly used by AFS clients to contact AFS cells outside their local domain. A subtype of this record is used by the obsolete DCE/DFS file system.
    ///
    /// [1183](https://datatracker.ietf.org/doc/html/rfc1183)
    AFSDB,
    /// Address Prefix List
    ///
    /// Specify lists of address ranges, e.g. in CIDR format, for various address families. Experimental.
    ///
    /// [3123](https://datatracker.ietf.org/doc/html/rfc3123)
    APL,
    /// Certification Authority Authorization
    ///
    /// DNS Certification Authority Authorization, constraining acceptable CAs for a host/domain
    ///
    /// [6844](https://datatracker.ietf.org/doc/html/rfc6844)
    CAA,
    /// Child copy of DNSKEY record, for transfer to parent
    ///
    /// [7344](https://datatracker.ietf.org/doc/html/rfc7344)
    CDNSKEY,
    /// Child DS
    ///
    /// Child copy of DS record, for transfer to parent
    ///
    /// [7344](https://datatracker.ietf.org/doc/html/rfc7344)
    CDS,
    /// Certificate record
    ///
    /// Stores PKIX, SPKI, PGP, etc.
    ///
    /// [4398](https://datatracker.ietf.org/doc/html/rfc4398)
    CERT,
    /// Canonical name record
    ///
    /// Alias of one name to another: the DNS lookup will continue by retrying the lookup with the new name.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    CNAME,
    /// Child-to-Parent Synchronization
    ///
    /// Specify a synchronization mechanism between a child and a parent DNS zone. Typical example is declaring the same NS records in the parent and the child zone
    ///
    /// [7477](https://datatracker.ietf.org/doc/html/rfc7477)
    CSYNC,
    /// DHCP identifier
    ///
    /// Used in conjunction with the FQDN option to DHCP
    ///
    /// [4701](https://datatracker.ietf.org/doc/html/rfc4701)
    DHCID,
    /// DNSSEC Lookaside Validation record
    ///
    /// For publishing DNSSEC trust anchors outside of the DNS delegation chain. Uses the same format as the DS record. RFC 5074 describes a way of using these records.
    ///
    /// [4431](https://datatracker.ietf.org/doc/html/rfc4431)
    DLV,
    /// Delegation name record
    ///
    /// Alias for a name and all its subnames, unlike CNAME, which is an alias for only the exact name. Like a CNAME record, the DNS lookup will continue by retrying the lookup with the new name.
    ///
    /// [6672](https://datatracker.ietf.org/doc/html/rfc6672)
    DNAME,
    /// DNS Key record
    ///
    /// The key record used in DNSSEC. Uses the same format as the KEY record.
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    DNSKEY,
    /// Delegation signer
    ///
    /// The record used to identify the DNSSEC signing key of a delegated zone
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    DS,
    /// MAC address (EUI-48)
    ///
    /// A 48-bit IEEE Extended Unique Identifier.
    ///
    /// [7043](https://datatracker.ietf.org/doc/html/rfc7043)
    EUI48,
    /// MAC address (EUI-64)
    ///
    /// A 64-bit IEEE Extended Unique Identifier.
    ///
    /// [7043](https://datatracker.ietf.org/doc/html/rfc7043)
    EUI64,
    /// Host Information
    ///
    /// Providing Minimal-Sized Responses to DNS Queries That Have QTYPE=ANY
    ///
    /// [8482](https://datatracker.ietf.org/doc/html/rfc8482)
    HINFO,
    /// Host Identity Protocol
    ///
    /// Method of separating the end-point identifier and locator roles of IP addresses.
    ///
    /// [8005](https://datatracker.ietf.org/doc/html/rfc8005)
    HIP,
    /// HTTPS Binding
    ///
    /// RR that improves performance for clients that need to resolve many resources to access a domain.
    ///
    /// [9460](https://datatracker.ietf.org/doc/html/rfc9460)
    HTTPS,
    /// IPsec Key
    ///
    /// Key record that can be used with IPsec
    ///
    /// [4025](https://datatracker.ietf.org/doc/html/rfc4025)
    IPSECKEY,
    /// Key record
    ///
    /// Used only for SIG(0) (RFC 2931) and TKEY (RFC 2930).[5] RFC 3445 eliminated their use for application keys and limited their use to DNSSEC.[6] RFC 3755 designates DNSKEY as the replacement within DNSSEC.[7] RFC 4025 designates IPSECKEY as the replacement for use with IPsec.[8]
    ///
    /// [2535](https://datatracker.ietf.org/doc/html/rfc2535)
    /// [2930](https://datatracker.ietf.org/doc/html/rfc2930)
    KEY,
    /// Key Exchanger record
    ///
    /// Used with some cryptographic systems (not including DNSSEC) to identify a key management agent for the associated domain-name. Note that this has nothing to do with DNS Security. It is Informational status, rather than being on the IETF standards-track. It has always had limited deployment, but is still in use.
    ///
    /// [2230](https://datatracker.ietf.org/doc/html/rfc2230)
    KX,
    /// Location record
    ///
    /// Specifies a geographical location associated with a domain name
    ///
    /// [1876](https://datatracker.ietf.org/doc/html/rfc1876)
    LOC,
    /// Mail exchange record
    ///
    /// List of mail exchange servers that accept email for a domain
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    /// [7505](https://datatracker.ietf.org/doc/html/rfc7505)
    MX,
    /// Naming Authority Pointer
    ///
    /// Allows regular-expression-based rewriting of domain names which can then be used as URIs, further domain names to lookups, etc.
    ///
    /// [3403](https://datatracker.ietf.org/doc/html/rfc3403)
    NAPTR,
    /// Name server record
    ///
    /// Delegates a DNS zone to use the given authoritative name servers
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    NS,
    /// Next Secure record
    ///
    /// Part of DNSSEC—used to prove a name does not exist. Uses the same format as the (obsolete) NXT record.
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    NSEC,
    /// Next Secure record version 3
    ///
    /// An extension to DNSSEC that allows proof of nonexistence for a name without permitting zonewalking
    ///
    /// [5155](https://datatracker.ietf.org/doc/html/rfc5155)
    NSEC3,
    /// NSEC3 parameters
    ///
    /// Parameter record for use with NSEC3
    ///
    /// [5155](https://datatracker.ietf.org/doc/html/rfc5155)
    NSEC3PARAM,
    /// OpenPGP public key record
    ///
    /// A DNS-based Authentication of Named Entities (DANE) method for publishing and locating OpenPGP public keys in DNS for a specific email address using an OPENPGPKEY DNS resource record.
    ///
    /// [7929](https://datatracker.ietf.org/doc/html/rfc7929)
    OPENPGPKEY,
    /// PTR Resource Record
    ///
    /// Pointer to a canonical name. Unlike a CNAME, DNS processing stops and just the name is returned. The most common use is for implementing reverse DNS lookups, but other uses include such things as DNS-SD.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    PTR,
    /// DNSSEC signature
    ///
    /// Signature for a DNSSEC-secured record set. Uses the same format as the SIG record.
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    RRSIG,
    /// Responsible Person
    ///
    /// Information about the responsible person(s) for the domain. Usually an email address with the @ replaced by a .
    ///
    /// [1183](https://datatracker.ietf.org/doc/html/rfc1183)
    RP,
    /// Signature
    ///
    /// Signature record used in SIG(0) (RFC 2931) and TKEY (RFC 2930).[7] RFC 3755 designated RRSIG as the replacement for SIG for use within DNSSEC.[7]
    ///
    /// [2535](https://datatracker.ietf.org/doc/html/rfc2535)
    SIG,
    /// S/MIME cert association
    ///
    /// Associates an S/MIME certificate with a domain name for sender authentication.
    ///
    /// [8162](https://datatracker.ietf.org/doc/html/rfc8162)
    SMIMEA,
    /// Start of [a zone of] authority record
    ///
    /// Specifies authoritative information about a DNS zone, including the primary name server, the email of the domain administrator, the domain serial number, and several timers relating to refreshing the zone.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    /// [2308](https://datatracker.ietf.org/doc/html/rfc2308)
    SOA,
    /// Service locator
    ///
    /// Generalized service location record, used for newer protocols instead of creating protocol-specific records such as MX.
    ///
    /// [2782](https://datatracker.ietf.org/doc/html/rfc2782)
    SRV,
    /// SSH Public Key Fingerprint
    ///
    /// Resource record for publishing SSH public host key fingerprints in the DNS, in order to aid in verifying the authenticity of the host. RFC 6594 defines ECC SSH keys and SHA-256 hashes. See the IANA SSHFP RR parameters registry for details.
    ///
    /// [4255](https://datatracker.ietf.org/doc/html/rfc4255)
    SSHFP,
    /// Service Binding
    ///
    /// RR that improves performance for clients that need to resolve many resources to access a domain.
    ///
    /// [9460](https://datatracker.ietf.org/doc/html/rfc9460)
    SVCB,
    /// DNSSEC Trust Authorities
    ///
    /// Part of a deployment proposal for DNSSEC without a signed DNS root. See the IANA database and Weiler Spec for details. Uses the same format as the DS record.
    TA,
    /// Transaction Key record
    ///
    /// A method of providing keying material to be used with TSIG that is encrypted under the public key in an accompanying KEY RR.[12]
    ///
    /// [2930](https://datatracker.ietf.org/doc/html/rfc2930)
    TKEY,
    /// TLSA certificate association
    ///
    /// A record for DANE. RFC 6698 defines "The TLSA DNS resource record is used to associate a TLS server certificate or public key with the domain name where the record is found, thus forming a 'TLSA certificate association'".
    ///
    /// [6698](https://datatracker.ietf.org/doc/html/rfc6698)
    TLSA,
    /// Transaction Signature
    ///
    /// Can be used to authenticate dynamic updates as coming from an approved client, or to authenticate responses as coming from an approved recursive name server[13] similar to DNSSEC.
    ///
    /// [2845](https://datatracker.ietf.org/doc/html/rfc2845)
    TSIG,
    /// Text record
    ///
    /// Originally for arbitrary human-readable text in a DNS record. Since the early 1990s, however, this record more often carries machine-readable data, such as specified by RFC 1464, opportunistic encryption, Sender Policy Framework, DKIM, DMARC, DNS-SD, etc.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    TXT,
    /// Uniform Resource Identifier
    ///
    /// Can be used for publishing mappings from hostnames to URIs.
    ///
    /// [7553](https://datatracker.ietf.org/doc/html/rfc7553)
    URI,
    /// Message Digests for DNS Zones
    ///
    /// Provides a cryptographic message digest over DNS zone data at rest.
    ///
    /// [8976](https://datatracker.ietf.org/doc/html/rfc8976)
    ZONEMD,
}

impl Type {
    pub fn is_a(&self) -> bool {
        *self == Self::A
    }
    pub fn is_aaaa(&self) -> bool {
        *self == Self::AAAA
    }
    pub fn is_afsdb(&self) -> bool {
        *self == Self::AFSDB
    }
    pub fn is_apl(&self) -> bool {
        *self == Self::APL
    }
    pub fn is_caa(&self) -> bool {
        *self == Self::CAA
    }
    pub fn is_cdnskey(&self) -> bool {
        *self == Self::CDNSKEY
    }
    pub fn is_cds(&self) -> bool {
        *self == Self::CDS
    }
    pub fn is_cert(&self) -> bool {
        *self == Self::CERT
    }
    pub fn is_cname(&self) -> bool {
        *self == Self::CNAME
    }
    pub fn is_csync(&self) -> bool {
        *self == Self::CSYNC
    }
    pub fn is_dhcid(&self) -> bool {
        *self == Self::DHCID
    }
    pub fn is_dlv(&self) -> bool {
        *self == Self::DLV
    }
    pub fn is_dname(&self) -> bool {
        *self == Self::DNAME
    }
    pub fn is_dnskey(&self) -> bool {
        *self == Self::DNSKEY
    }
    pub fn is_ds(&self) -> bool {
        *self == Self::DS
    }
    pub fn is_eui48(&self) -> bool {
        *self == Self::EUI48
    }
    pub fn is_eui64(&self) -> bool {
        *self == Self::EUI64
    }
    pub fn is_hinfo(&self) -> bool {
        *self == Self::HINFO
    }
    pub fn is_hip(&self) -> bool {
        *self == Self::HIP
    }
    pub fn is_https(&self) -> bool {
        *self == Self::HTTPS
    }
    pub fn is_ipseckey(&self) -> bool {
        *self == Self::IPSECKEY
    }
    pub fn is_key(&self) -> bool {
        *self == Self::KEY
    }
    pub fn is_kx(&self) -> bool {
        *self == Self::KX
    }
    pub fn is_loc(&self) -> bool {
        *self == Self::LOC
    }
    pub fn is_mx(&self) -> bool {
        *self == Self::MX
    }
    pub fn is_naptr(&self) -> bool {
        *self == Self::NAPTR
    }
    pub fn is_ns(&self) -> bool {
        *self == Self::NS
    }
    pub fn is_nsec(&self) -> bool {
        *self == Self::NSEC
    }
    pub fn is_nsec3(&self) -> bool {
        *self == Self::NSEC3
    }
    pub fn is_nsec3param(&self) -> bool {
        *self == Self::NSEC3PARAM
    }
    pub fn is_openpgpkey(&self) -> bool {
        *self == Self::OPENPGPKEY
    }
    pub fn is_ptr(&self) -> bool {
        *self == Self::PTR
    }
    pub fn is_rrsig(&self) -> bool {
        *self == Self::RRSIG
    }
    pub fn is_rp(&self) -> bool {
        *self == Self::RP
    }
    pub fn is_sig(&self) -> bool {
        *self == Self::SIG
    }
    pub fn is_smimea(&self) -> bool {
        *self == Self::SMIMEA
    }
    pub fn is_soa(&self) -> bool {
        *self == Self::SOA
    }
    pub fn is_srv(&self) -> bool {
        *self == Self::SRV
    }
    pub fn is_sshfp(&self) -> bool {
        *self == Self::SSHFP
    }
    pub fn is_svcb(&self) -> bool {
        *self == Self::SVCB
    }
    pub fn is_ta(&self) -> bool {
        *self == Self::TA
    }
    pub fn is_tkey(&self) -> bool {
        *self == Self::TKEY
    }
    pub fn is_tlsa(&self) -> bool {
        *self == Self::TLSA
    }
    pub fn is_tsig(&self) -> bool {
        *self == Self::TSIG
    }
    pub fn is_txt(&self) -> bool {
        *self == Self::TXT
    }
    pub fn is_uri(&self) -> bool {
        *self == Self::URI
    }
    pub fn is_zonemd(&self) -> bool {
        *self == Self::ZONEMD
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => f.write_str("A"),
            Self::AAAA => f.write_str("AAAA"),
            Self::AFSDB => f.write_str("AFSDB"),
            Self::APL => f.write_str("APL"),
            Self::CAA => f.write_str("CAA"),
            Self::CDNSKEY => f.write_str("CDNSKEY"),
            Self::CDS => f.write_str("CDS"),
            Self::CERT => f.write_str("CERT"),
            Self::CNAME => f.write_str("CNAME"),
            Self::CSYNC => f.write_str("CSYNC"),
            Self::DHCID => f.write_str("DHCID"),
            Self::DLV => f.write_str("DLV"),
            Self::DNAME => f.write_str("DNAME"),
            Self::DNSKEY => f.write_str("DNSKEY"),
            Self::DS => f.write_str("DS"),
            Self::EUI48 => f.write_str("EUI48"),
            Self::EUI64 => f.write_str("EUI64"),
            Self::HINFO => f.write_str("HINFO"),
            Self::HIP => f.write_str("HIP"),
            Self::HTTPS => f.write_str("HTTPS"),
            Self::IPSECKEY => f.write_str("IPSECKEY"),
            Self::KEY => f.write_str("KEY"),
            Self::KX => f.write_str("KX"),
            Self::LOC => f.write_str("LOC"),
            Self::MX => f.write_str("MX"),
            Self::NAPTR => f.write_str("NAPTR"),
            Self::NS => f.write_str("NS"),
            Self::NSEC => f.write_str("NSEC"),
            Self::NSEC3 => f.write_str("NSEC3"),
            Self::NSEC3PARAM => f.write_str("NSEC3PARAM"),
            Self::OPENPGPKEY => f.write_str("OPENPGPKEY"),
            Self::PTR => f.write_str("PTR"),
            Self::RRSIG => f.write_str("RRSIG"),
            Self::RP => f.write_str("RP"),
            Self::SIG => f.write_str("SIG"),
            Self::SMIMEA => f.write_str("SMIMEA"),
            Self::SOA => f.write_str("SOA"),
            Self::SRV => f.write_str("SRV"),
            Self::SSHFP => f.write_str("SSHFP"),
            Self::SVCB => f.write_str("SVCB"),
            Self::TA => f.write_str("TA"),
            Self::TKEY => f.write_str("TKEY"),
            Self::TLSA => f.write_str("TLSA"),
            Self::TSIG => f.write_str("TSIG"),
            Self::TXT => f.write_str("TXT"),
            Self::URI => f.write_str("URI"),
            Self::ZONEMD => f.write_str("ZONEMD"),
        }
    }
}
