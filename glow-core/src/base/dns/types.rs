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
