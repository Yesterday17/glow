use glow_common::traits::GetU16Value;

/// References: https://en.wikipedia.org/wiki/List_of_DNS_record_types and RFCs

/// TYPE fields are used in resource records.
/// Note that these types are a subset of QTYPEs.
#[derive(Copy, Clone)]
pub enum Type {
    /// 1 a host address
    A = 1,
    /// 2 an authoritative name server
    NS = 2,
    /// 3 a mail destination (Obsolete - use MX)
    #[deprecated = "Obsolete - use MX"]
    MD = 3,
    /// 4 a mail forwarder (Obsolete - use MX)
    #[deprecated = "Obsolete - use MX"]
    MF = 4,
    /// 5 the canonical name for an alias
    CNAME = 5,
    /// 6 marks the start of a zone of authority
    SOA = 6,
    /// 7 a mailbox domain name (EXPERIMENTAL)
    MB = 7,
    /// 8 a mail group member (EXPERIMENTAL)
    MG = 8,
    /// 9 a mail rename domain name (EXPERIMENTAL)
    MR = 9,
    /// 10 a null RR (EXPERIMENTAL)
    NULL = 10,
    /// 11 a well known service description
    #[deprecated = "Recommend against using WKS records from DNS(RFC 1127)"]
    WKS = 11,
    /// 12 a domain name pointer
    PTR = 12,
    /// 13 host information
    HINFO = 13,
    /// 14 mailbox or mail list information
    MINFO = 14,
    /// 15 mail exchange
    MX = 15,
    /// 16 text strings
    TXT = 16,
    /// 17 response person
    RP = 17,
    /// 18 AFS database record
    AFSDB = 18,
    /// 19 X25
    X25 = 19,
    /// 20 integrated service digital network
    ISDN = 20,
    /// 21 route through
    RT = 21,
    /// 22 network service access point
    NSAP = 22,
    /// 23 NASP-PTR
    #[allow(non_camel_case_types)]
    NSAP_PTR = 23,
    /// 24 signature
    SIG = 24,
    /// 25
    KEY = 25,
    /// 26
    PX = 26,
    /// 27
    GPOS = 27,
    /// 28 ipv6 address record
    AAAA = 28,
    /// 29 location record
    LOC = 29,
}

impl GetU16Value for Type {
    fn value(&self) -> u16 {
        *self as u16
    }
}

/// QTYPE fields appear in the question part of a query. QTYPES are a
/// superset of TYPEs, hence all TYPEs are valid QTYPEs. In addition, the
/// following QTYPEs are defined:
pub enum QType {
    Type(Type),
    /// 252 A request for a transfer of an entire zone
    AXFR,
    /// 253 A request for mailbox-related records (MB, MG or MR)
    MAILB,
    /// 254 A request for mail agent RRs (Obsolete - see MX)
    #[deprecated = "Obsolete - see MX"]
    MAILA,
    /// 255 A request for all records
    ANY,
}

impl GetU16Value for QType {
    fn value(&self) -> u16 {
        match self {
            QType::Type(t) => t.value(),
            QType::AXFR => 252,
            QType::MAILB => 253,
            #[allow(deprecated)]
            QType::MAILA => 254,
            QType::ANY => 255,
        }
    }
}
