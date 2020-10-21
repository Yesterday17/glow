use glow_common::traits::GetU16Value;

/// CLASS fields appear in resource records. The following CLASS mnemonics
/// and values are defined:
#[derive(Copy, Clone)]
pub enum Class {
    /// 1 the Internet
    IN = 1,
    /// 2 the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    #[deprecated = "Obsoltete - used only for examples in some obsolete RFCs"]
    CS = 2,
    /// 3 the CHAOS class
    CH = 3,
    /// 4 Hesiod [Dyer 87]
    HS = 4,
}

impl GetU16Value for Class {
    fn value(&self) -> u16 {
        *self as u16
    }
}

/// QCLASS fields appear in the question section of a query. QCLASS values
/// are a superset of CLASS values; every CLASS is a valid QCLASS. In
/// addition to CLASS values, the following QCLASSes are defined:
pub enum QClass {
    Class(Class),
    /// 255 any class
    // #[allow(non_camel_case_types)]
    ANY,
}

impl GetU16Value for QClass {
    fn value(&self) -> u16 {
        match self {
            QClass::Class(c) => c.value(),
            QClass::ANY => 255,
        }
    }
}
