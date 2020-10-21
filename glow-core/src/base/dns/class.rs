/// CLASS fields appear in resource records. The following CLASS mnemonics
/// and values are defined:
#[derive(Copy, Clone, Debug)]
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

impl Into<u16> for Class {
    fn into(self) -> u16 {
        self as u16
    }
}

impl From<u16> for Class {
    fn from(raw: u16) -> Self {
        match raw {
            1 => Class::IN,
            #[allow(deprecated)]
            2 => Class::CS,
            3 => Class::CH,
            4 => Class::HS,
            _ => Class::IN, // FIXME
        }
    }
}

/// QCLASS fields appear in the question section of a query. QCLASS values
/// are a superset of CLASS values; every CLASS is a valid QCLASS. In
/// addition to CLASS values, the following QCLASSes are defined:
#[derive(Copy, Clone, Debug)]
pub enum QClass {
    Class(Class),
    /// 255 any class
    // #[allow(non_camel_case_types)]
    ANY,
}

impl Into<u16> for QClass {
    fn into(self) -> u16 {
        match self {
            QClass::Class(c) => c.into(),
            QClass::ANY => 255,
        }
    }
}

impl From<u16> for QClass {
    fn from(raw: u16) -> Self {
        match raw {
            255 => QClass::ANY,
            _ => QClass::Class(Class::from(raw)),
        }
    }
}
