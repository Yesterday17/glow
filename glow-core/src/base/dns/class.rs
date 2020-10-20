/// CLASS fields appear in resource records. The following CLASS mnemonics
/// and values are defined:
pub type Class = u16;
/// 1 the Internet
pub const IN: Class = 1;
/// 2 the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
#[deprecated = "Obsoltete - used only for examples in some obsolete RFCs"]
pub const CS: Class = 2;
/// 3 the CHAOS class
pub const CH: Class = 3;
/// 4 Hesiod [Dyer 87]
pub const HS: Class = 4;

/// QCLASS fields appear in the question section of a query. QCLASS values
/// are a superset of CLASS values; every CLASS is a valid QCLASS. In
/// addition to CLASS values, the following QCLASSes are defined:
pub type QClass = Class;
/// 255 any class
pub const ANY: QClass = 255;
