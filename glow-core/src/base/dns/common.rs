use super::{class, types};
use glow_utils::{get_bit, get_bits, set0, set1, u8_merge};

pub struct Header {
    /// A 16 bit identifier assigned by the program that
    /// generates any kind of query. This identifier is copied
    /// the corresponding reply and can be used by the requester
    /// to match up replies to outstanding queries.
    pub id: u16,

    /// header flags, detail described below
    flag: HeaderFlag,

    /// an unsigned 16 bit integer specifying the number of
    /// entries in the question section.
    pub qd_count: u16,

    /// an unsigned 16 bit integer specifying the number of
    /// resource records in the answer section.
    pub an_count: u16,

    /// an unsigned 16 bit integer specifying the number of name
    /// server resource records in the authority records
    /// section.
    pub ns_count: u16,

    /// an unsigned 16 bit integer specifying the number of
    /// resource records in the additional records section.
    pub ar_count: u16,
}

impl Header {
    pub fn from(raw: [u8; 12]) -> Header {
        Header {
            id: u8_merge!(raw[0], raw[1]),
            flag: u8_merge!(raw[2], raw[3]),
            qd_count: u8_merge!(raw[4], raw[5]),
            an_count: u8_merge!(raw[6], raw[7]),
            ns_count: u8_merge!(raw[8], raw[9]),
            ar_count: u8_merge!(raw[10], raw[11]),
        }
    }

    pub fn from_vec(raw: Vec<u8>) -> Option<Header> {
        if raw.len() < 12 {
            None
        } else {
            let mut v: [u8; 12] = [0; 12];
            for i in 0..12 {
                v[i] = raw[i];
            }
            Some(Header::from(v))
        }
    }

    pub fn is_query(&self) -> bool {
        get_bit!(self.flag, 0, u16) == 0
    }

    pub fn is_response(&self) -> bool {
        get_bit!(self.flag, 0, u16) == 1
    }
}

type HeaderFlag = u16;

pub struct HeaderFlagBuilder {
    flag: u16,
}

impl HeaderFlagBuilder {
    pub fn new() -> HeaderFlagBuilder {
        HeaderFlagBuilder { flag: 0 }
    }

    pub fn build(&self) -> HeaderFlag {
        self.flag
    }

    /// A one bit field that specifies whether this message is a
    ///
    /// query (0)
    pub fn qr_query(&mut self) -> &HeaderFlagBuilder {
        set1!(self.flag, 0, u16);
        self
    }
    /// or a response (1).
    pub fn qr_response(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 0, u16);
        self
    }

    /// A four bit field that specifies kind of query in this
    /// message. This value is set by the originator of a query
    /// and copied into the response. The values are:
    ///
    /// 0 a standard query (QUERY)
    pub fn op_standard_query(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 1, u16); // 0
        set0!(self.flag, 2, u16); // 0
        set0!(self.flag, 3, u16); // 0
        set0!(self.flag, 4, u16); // 0
        self
    }
    /// 1 an inverse query (IQUERY)
    pub fn op_inverse_query(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 1, u16); // 0
        set0!(self.flag, 2, u16); // 0
        set0!(self.flag, 3, u16); // 0
        set1!(self.flag, 4, u16); // 1
        self
    }
    /// 2 a server status request (STATUS)
    pub fn op_status_request(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 1, u16); // 0
        set0!(self.flag, 2, u16); // 0
        set1!(self.flag, 3, u16); // 1
        set0!(self.flag, 4, u16); // 0
        self
    }
    /// 3-15 reserved for future use
    pub fn op_reversed(&mut self) -> &HeaderFlagBuilder {
        set1!(self.flag, 1, u16); // 1
        set1!(self.flag, 2, u16); // 1
        set1!(self.flag, 3, u16); // 1
        set1!(self.flag, 4, u16); // 1
        self
    }

    /// Authoritative Answer - this bit is valid in responses,
    /// and specifies that the responding name server is an
    /// authority for the domain name in question section.
    ///
    /// Note that the contents of the answer section may have
    /// multiple owner names because of aliases. The AA bit
    /// corresponds to the name which matches the query name, or
    /// the first owner name in the answer section.
    pub fn aa_on(&mut self) -> &HeaderFlagBuilder {
        set1!(self.flag, 5, u16);
        self
    }
    pub fn aa_off(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 5, u16);
        self
    }

    /// TrunCation - specifies that this message was truncated
    /// due to length greater than that permitted on the
    /// transmission channel.
    pub fn tc_on(&mut self) -> &HeaderFlagBuilder {
        set1!(self.flag, 6, u16);
        self
    }

    /// Recursion Desired - this bit may be set in a query and
    /// is copied into the response. If RD is set, it directs
    /// the name server to pursue the query recursively.
    /// Recursive query support is optional.
    pub fn rd_on(&mut self) -> &HeaderFlagBuilder {
        set1!(self.flag, 7, u16);
        self
    }

    /// Recursion Available - this be is set or cleared in a
    /// response, and denotes whether recursive query support is
    /// available in the name server.
    pub fn ra_on(&mut self) -> &HeaderFlagBuilder {
        set1!(self.flag, 8, u16);
        self
    }

    /// Reserved for future use. Must be zero in all queries
    /// and responses.
    pub fn zf(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 9, u16);
        set0!(self.flag, 10, u16);
        set0!(self.flag, 11, u16);
        self
    }

    /// Response code - this 4 bit field is set as part of
    /// responses. The values have the following
    /// interpretation:
    ///
    /// 0 No error condition
    pub fn rcode_no_error(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 12, u16);
        set0!(self.flag, 13, u16);
        set0!(self.flag, 14, u16);
        set0!(self.flag, 15, u16);
        self
    }
    /// 1 Format error - The name server was
    /// unable to interpret the query.
    pub fn rcode_format_error(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 12, u16);
        set0!(self.flag, 13, u16);
        set0!(self.flag, 14, u16);
        set1!(self.flag, 15, u16);
        self
    }
    /// 2 Server failure - The name server was
    /// unable to process this query due to a
    /// problem with the name server.
    pub fn rcode_server_failure(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 12, u16);
        set0!(self.flag, 13, u16);
        set1!(self.flag, 14, u16);
        set0!(self.flag, 15, u16);
        self
    }
    /// 3 Name Error - Meaningful only for
    /// responses from an authoritative name
    /// server, this code signifies that the
    /// domain name referenced in the query does
    /// not exist.
    pub fn rcode_name_error(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 12, u16);
        set0!(self.flag, 13, u16);
        set1!(self.flag, 14, u16);
        set1!(self.flag, 15, u16);
        self
    }
    /// 4 Not Implemented - The name server does
    /// not support the requested kind of query.
    pub fn rcode_not_implemented(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 12, u16);
        set1!(self.flag, 13, u16);
        set0!(self.flag, 14, u16);
        set0!(self.flag, 15, u16);
        self
    }
    /// 5 Refused - The name server refuses to
    /// perform the specified operation for
    /// policy reasons. For example, a name
    /// server may not wish to provide the
    /// information to the particular requester,
    /// or a name server may not wish to perform
    /// a particular operation (e.g., zone
    /// transfer) for particular data.
    pub fn rcode_refused(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 12, u16);
        set1!(self.flag, 13, u16);
        set0!(self.flag, 14, u16);
        set1!(self.flag, 15, u16);
        self
    }
    /// 6-15 Reserved for future use.
    pub fn rcode_reversed(&mut self) -> &HeaderFlagBuilder {
        set0!(self.flag, 12, u16);
        set1!(self.flag, 13, u16);
        set1!(self.flag, 14, u16);
        set0!(self.flag, 15, u16);
        self
    }
}

struct Question {
    /// a domain name represented as a sequence of labels, where
    /// each label consists of a length octet followed by that
    /// number of octets. The domain name terminates with the
    /// zero length octet for the null label of the root. Note
    /// that this field may be an odd number of octets; no
    /// padding is used.
    q_name: String,

    /// a two octet code which specifies the type of the query.
    /// The values for this field include all codes valid for a
    /// TYPE field, together with some more general codes which
    /// can match more than one type of RR.
    q_type: types::QueryType,

    /// a two octet code that specifies the class of the query.
    /// For example, the QCLASS field is IN for the Internet.
    q_class: class::QClass,
}

struct ResourceRecord {
    /// a domain name to which this resource record pertains.
    name: String,

    /// two octets containing one of the RR type codes. This
    /// field specifies the meaning of the data in the RDATA
    /// field.
    r_type: types::RecordType,

    /// two octets which specify the class of the data in the
    /// RDATA field.
    class: class::Class,

    /// a 32 bit unsigned integer that specifies the time
    /// interval (in seconds) that the resource record may be
    /// cached before it should be discarded. Zero values are
    /// interpreted to mean that the RR can only be used for the
    /// transaction in progress, and should not be cached.
    ttl: u32,

    /// an unsigned 16 bit integer that specifies the length in
    /// octets of the RDATA field.
    rd_length: u16,

    /// a variable length string of octets that describes the
    /// resource. The format of this information varies
    /// according to the TYPE and CLASS of the resource record.
    /// For example, the if the TYPE is A and the CLASS is IN,
    /// the RDATA field is a 4 octet ARPA Internet address.
    r_data: Vec<u8>, // FIXME
}
