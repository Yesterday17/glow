/// References: https://en.wikipedia.org/wiki/List_of_DNS_record_types and RFCs

/// TYPE fields are used in resource records.
/// Note that these types are a subset of QTYPEs.
pub type RecordType = u16;
/// 1 a host address
pub const A: RecordType = 1;
/// 2 an authoritative name server
pub const NS: RecordType = 2;
/// 3 a mail destination (Obsolete - use MX)
#[deprecated = "Obsolete - use MX"]
pub const MD: RecordType = 3;
/// 4 a mail forwarder (Obsolete - use MX)
#[deprecated = "Obsolete - use MX"]
pub const MF: RecordType = 4;
/// 5 the canonical name for an alias
pub const CNAME: RecordType = 5;
/// 6 marks the start of a zone of authority
pub const SOA: RecordType = 6;
/// 7 a mailbox domain name (EXPERIMENTAL)
pub const MB: RecordType = 7;
/// 8 a mail group member (EXPERIMENTAL)
pub const MG: RecordType = 8;
/// 9 a mail rename domain name (EXPERIMENTAL)
pub const MR: RecordType = 9;
/// 10 a null RR (EXPERIMENTAL)
pub const NULL: RecordType = 10;
/// 11 a well known service description
#[deprecated = "Recommend against using WKS records from DNS(RFC 1127)"]
pub const WKS: RecordType = 11;
/// 12 a domain name pointer
pub const PTR: RecordType = 12;
/// 13 host information
pub const HINFO: RecordType = 13;
/// 14 mailbox or mail list information
pub const MINFO: RecordType = 14;
/// 15 mail exchange
pub const MX: RecordType = 15;
/// 16 text strings
pub const TXT: RecordType = 16;
/// 17 response person
pub const RP: RecordType = 17;
/// 18 AFS database record
pub const AFSDB: RecordType = 18;
/// 19 X25
pub const X25: RecordType = 19;
/// 20 integrated service digital network
pub const ISDN: RecordType = 20;
/// 21 route through
pub const RT: RecordType = 21;
/// 22 network service access point
pub const NSAP: RecordType = 22;
/// 23 NASP-PTR
pub const NSAP_PTR: RecordType = 23;
/// 24 signature
pub const SIG: RecordType = 24;
/// 25
pub const KEY: RecordType = 25;
/// 26
pub const PX: RecordType = 26;
/// 27
pub const GPOS: RecordType = 27;
/// 28 ipv6 address record
pub const AAAA: RecordType = 28;
/// 29 location record
pub const LOC: RecordType = 29;

/// QTYPE fields appear in the question part of a query. QTYPES are a
/// superset of TYPEs, hence all TYPEs are valid QTYPEs. In addition, the
/// following QTYPEs are defined:
pub type QueryType = RecordType;
/// 252 A request for a transfer of an entire zone
pub const AXFR: QueryType = 252;
/// 253 A request for mailbox-related records (MB, MG or MR)
pub const MAILB: QueryType = 253;
/// 254 A request for mail agent RRs (Obsolete - see MX)
#[deprecated = "Obsolete - see MX"]
pub const MAILA: QueryType = 254;
/// 255 A request for all records
pub const ANY: QueryType = 255;
