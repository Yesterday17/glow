use super::{class, types};
use bytes::{BufMut, BytesMut};
use glow_utils::{get_bit, get_bits, u8_merge};
use std::net::{Ipv4Addr, Ipv6Addr};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Copy, Clone)]
pub struct Header {
    /// A 16 bit identifier assigned by the program that
    /// generates any kind of query. This identifier is copied
    /// the corresponding reply and can be used by the requester
    /// to match up replies to outstanding queries.
    pub id: u16,

    /// header flags, detail described below
    flag: u16,

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
    pub fn new(
        id: u16,
        flag: u16,
        qd_count: u16,
        an_count: u16,
        ns_count: u16,
        ar_count: u16,
    ) -> Header {
        Header {
            id,
            flag,
            qd_count,
            an_count,
            ns_count,
            ar_count,
        }
    }

    pub fn flag(&self) -> HeaderFlag {
        HeaderFlag::from(self.flag)
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new(rand::random(), HeaderFlag::DEFAULT_QUERY_FLAG, 0, 0, 0, 0)
    }
}

impl From<&[u8]> for Header {
    fn from(raw: &[u8]) -> Self {
        Header {
            id: u8_merge!(raw[0], raw[1]),
            flag: u8_merge!(raw[2], raw[3]),
            qd_count: u8_merge!(raw[4], raw[5]),
            an_count: u8_merge!(raw[6], raw[7]),
            ns_count: u8_merge!(raw[8], raw[9]),
            ar_count: u8_merge!(raw[10], raw[11]),
        }
    }
}

impl From<Vec<u8>> for Header {
    fn from(raw: Vec<u8>) -> Self {
        assert!(raw.len() >= 12);
        let mut v: [u8; 12] = [0; 12];
        for i in 0..12 {
            v[i] = raw[i];
        }
        Header::from(&v[..])
    }
}

impl Into<BytesMut> for Header {
    fn into(self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(12);
        buf.put_u16(self.id);
        buf.put_u16(self.flag);
        buf.put_u16(self.qd_count);
        buf.put_u16(self.an_count);
        buf.put_u16(self.ns_count);
        buf.put_u16(self.ar_count);
        buf
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct HeaderFlag {
    /// A one bit field that specifies whether this message
    /// is a query (0) or a response (1).
    pub is_response: bool,

    pub opcode: FlagOpCode,

    /// Authoritative Answer - this bit is valid in responses,
    /// and specifies that the responding name server is an
    /// authority for the domain name in question section.
    ///
    /// Note that the contents of the answer section may have
    /// multiple owner names because of aliases. The AA bit
    /// corresponds to the name which matches the query name, or
    /// the first owner name in the answer section.
    pub authoritative_answer: bool,

    /// TrunCation - specifies that this message was truncated
    /// due to length greater than that permitted on the
    /// transmission channel.
    pub truncated: bool,

    /// Recursion Desired - this bit may be set in a query and
    /// is copied into the response. If RD is set, it directs
    /// the name server to pursue the query recursively.
    /// Recursive query support is optional.
    pub recursion_desired: bool,

    /// Recursion Available - this be is set or cleared in a
    /// response, and denotes whether recursive query support is
    /// available in the name server.
    pub recursion_available: bool,

    pub rcode: FlagRCode,
}

impl From<u16> for HeaderFlag {
    fn from(flag: u16) -> Self {
        HeaderFlag {
            is_response: get_bit!(flag, 0, u16) == 1,
            opcode: FlagOpCode::from(get_bits!(flag, 1, 4, u16)),
            authoritative_answer: get_bit!(flag, 5, u16) == 1,
            truncated: get_bit!(flag, 6, u16) == 1,
            recursion_desired: get_bit!(flag, 7, u16) == 1,
            recursion_available: get_bit!(flag, 8, u16) == 1,
            rcode: FlagRCode::from(get_bits!(flag, 12, 4, u16)),
        }
    }
}

impl Default for HeaderFlag {
    fn default() -> Self {
        HeaderFlag {
            is_response: false,
            opcode: FlagOpCode::Query,
            authoritative_answer: false,
            truncated: false,
            recursion_desired: true,
            recursion_available: false,
            rcode: FlagRCode::NoError,
        }
    }
}

impl HeaderFlag {
    pub const DEFAULT_QUERY_FLAG: u16 = 0b0000000100000000;
}

/// A four bit field that specifies kind of query in this
/// message. This value is set by the originator of a query
/// and copied into the response. The values are:
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum FlagOpCode {
    /// 0 a standard query (QUERY)
    Query = 0,
    /// 1 an inverse query (IQUERY)
    IQuery = 1,
    /// 2 a server status request (STATUS)
    Status = 2,
    /// 3-15 reserved for future use
    Reserved = 15,
}

impl From<u16> for FlagOpCode {
    fn from(code: u16) -> Self {
        match code {
            0 => FlagOpCode::Query,
            1 => FlagOpCode::IQuery,
            2 => FlagOpCode::Status,
            _ => FlagOpCode::Reserved,
        }
    }
}

/// Response code - this 4 bit field is set as part of
/// responses. The values have the following
/// interpretation:
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum FlagRCode {
    /// 0 No error condition
    NoError = 0,
    /// 1 Format error - The name server was
    /// unable to interpret the query.
    FormatError = 1,
    /// 2 Server failure - The name server was
    /// unable to process this query due to a
    /// problem with the name server.
    ServerFailure = 2,
    /// 3 Name Error - Meaningful only for
    /// responses from an authoritative name
    /// server, this code signifies that the
    /// domain name referenced in the query does
    /// not exist.
    NameError = 3,
    /// 4 Not Implemented - The name server does
    /// not support the requested kind of query.
    NotImplemented = 4,
    /// 5 Refused - The name server refuses to
    /// perform the specified operation for
    /// policy reasons. For example, a name
    /// server may not wish to provide the
    /// information to the particular requester,
    /// or a name server may not wish to perform
    /// a particular operation (e.g., zone
    /// transfer) for particular data.s
    Refused = 5,
    /// 6-15 Reserved for future use.
    Reserved = 15,
}

impl From<u16> for FlagRCode {
    fn from(code: u16) -> Self {
        match code {
            0 => FlagRCode::NoError,
            1 => FlagRCode::FormatError,
            2 => FlagRCode::ServerFailure,
            3 => FlagRCode::NameError,
            4 => FlagRCode::NotImplemented,
            5 => FlagRCode::Refused,
            _ => FlagRCode::Reserved,
        }
    }
}

impl Into<String> for FlagRCode {
    fn into(self) -> String {
        match self {
            FlagRCode::NoError => "NoError".to_owned(),
            FlagRCode::FormatError => "FormatError".to_owned(),
            FlagRCode::ServerFailure => "ServerFailure".to_owned(),
            FlagRCode::NameError => "NameError".to_owned(),
            FlagRCode::NotImplemented => "NotImplemented".to_owned(),
            FlagRCode::Refused => "Refused".to_owned(),
            FlagRCode::Reserved => "Reserved".to_owned(),
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Question {
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
    q_type: types::QType,

    /// a two octet code that specifies the class of the query.
    /// For example, the QCLASS field is IN for the Internet.
    q_class: class::QClass,
}

impl Question {
    pub fn new(host: &str, q_type: types::QType) -> Question {
        Question {
            q_name: host.to_owned(),
            q_type,
            q_class: class::QClass::Class(class::Class::IN),
        }
    }

    /// append Question to BytesMut for query
    pub fn append_to<'a>(&self, m: &'a mut BytesMut) -> &'a mut BytesMut {
        for part in self.q_name.split('.') {
            m.put_u8(part.len() as u8);
            m.put(part.as_bytes());
        }
        m.put_u8(0);
        m.put_u16(self.q_type.into());
        m.put_u16(self.q_class.into());
        m
    }

    /// append Question to BytesMut to interfere with GFW
    /// This Question must be the last question
    /// reference: https://blog.ddosolitary.org/posts/research-on-dns-packet-forgery-of-gfw/
    pub fn append_gfw<'a>(&self, m: &'a mut BytesMut) -> &'a mut BytesMut {
        m.put_u8(0xc0);
        m.put_u8(0x12);
        m.put_u16(self.q_type.into());
        m.put_u16(self.q_class.into());
        m
    }

    pub fn parse(raw: &[u8], base_offset: usize) -> (Question, usize) {
        let (name, size) = read_name(raw, base_offset);
        let offset = base_offset + size;
        (
            Question {
                q_name: name,
                q_type: types::QType::from(u8_merge!(raw[offset], raw[offset + 1])),
                q_class: class::QClass::from(u8_merge!(raw[offset + 2], raw[offset + 3])),
            },
            size + 4,
        )
    }
}

impl Into<BytesMut> for Question {
    fn into(self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(12);
        self.append_to(&mut buf);
        buf
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ResourceRecord {
    /// a domain name to which this resource record pertains.
    pub name: String,

    /// two octets containing one of the RR type codes. This
    /// field specifies the meaning of the data in the RDATA
    /// field.
    pub r_type: types::Type,

    /// two octets which specify the class of the data in the
    /// RDATA field.
    pub class: class::Class,

    /// a 32 bit unsigned integer that specifies the time
    /// interval (in seconds) that the resource record may be
    /// cached before it should be discarded. Zero values are
    /// interpreted to mean that the RR can only be used for the
    /// transaction in progress, and should not be cached.
    pub ttl: u32,

    /// an unsigned 16 bit integer that specifies the length in
    /// octets of the RDATA field.
    rd_length: u16,

    /// a variable length string of octets that describes the
    /// resource. The format of this information varies
    /// according to the TYPE and CLASS of the resource record.
    /// For example, the if the TYPE is A and the CLASS is IN,
    /// the RDATA field is a 4 octet ARPA Internet address.
    pub r_data: RData,
}

impl ResourceRecord {
    pub fn parse(raw: &[u8], base_offset: usize) -> (ResourceRecord, usize) {
        let (name, size) = read_name(raw, base_offset);
        let offset = base_offset + size;
        let r_type = types::Type::from(u8_merge!(raw[offset], raw[offset + 1]));
        (
            ResourceRecord {
                name,
                r_type,
                class: class::Class::from(u8_merge!(raw[offset + 2], raw[offset + 3])),
                ttl: 0,
                rd_length: u8_merge!(raw[offset + 8], raw[offset + 9]),
                r_data: RData::decode(r_type, raw, offset + 10),
            },
            size + 4,
        )
    }
}

// TODO: more RDATA types
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum RData {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    Unknown,
}

impl RData {
    pub fn encode(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        match self {
            RData::A(addr) => {
                let octets: [u8; 4] = addr.octets();
                for octet in octets.iter() {
                    buf.put_u8(*octet);
                }
            }
            RData::AAAA(addr) => {
                let octets: [u8; 16] = addr.octets();
                for octet in octets.iter() {
                    buf.put_u8(*octet);
                }
            }
            _ => {}
        };

        buf
    }

    fn decode(r_type: types::Type, raw: &[u8], offset: usize) -> RData {
        match r_type {
            types::Type::A => RData::A(Ipv4Addr::new(
                raw[offset],
                raw[offset + 1],
                raw[offset + 2],
                raw[offset + 3],
            )),
            types::Type::AAAA => RData::AAAA(Ipv6Addr::new(
                u8_merge!(raw[offset], raw[offset + 1]),
                u8_merge!(raw[offset + 2], raw[offset + 3]),
                u8_merge!(raw[offset + 4], raw[offset + 5]),
                u8_merge!(raw[offset + 6], raw[offset + 7]),
                u8_merge!(raw[offset + 8], raw[offset + 9]),
                u8_merge!(raw[offset + 10], raw[offset + 11]),
                u8_merge!(raw[offset + 12], raw[offset + 13]),
                u8_merge!(raw[offset + 14], raw[offset + 15]),
            )),
            _ => RData::Unknown,
        }
    }
}

fn read_name_part(raw: &[u8], offset: usize) -> (String, usize) {
    let len = raw[offset] as usize;
    (
        std::str::from_utf8(&raw[(offset + 1)..(offset + len + 1)])
            .unwrap()
            .to_owned(),
        len,
    )
}

fn read_name(raw: &[u8], base_offset: usize) -> (String, usize) {
    let mut name = String::new();
    let mut offset = base_offset;
    let mut len = raw[offset];
    while len != 0 {
        let is_ptr = len & 0b11000000 > 0;
        let (part, size) = if is_ptr {
            let offset = (len as usize & 0b00111111 << 8) + raw[offset + 1] as usize;
            read_name(raw, offset)
        } else {
            read_name_part(raw, offset)
        };
        name.push_str(&part);

        if is_ptr {
            offset += 1;
            break;
        } else {
            offset += size + 1;
            len = raw[offset];
            name.push('.');
        }
    }
    offset += 1; // final '\0'
    (name, offset - base_offset)
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Message {
    /// sections
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,

    /// control flags
    pub bypass_gfw: bool,
}

impl From<&[u8]> for Message {
    fn from(buffer: &[u8]) -> Self {
        // parse header
        let header = Header::from(buffer);

        match header.flag().rcode {
            FlagRCode::NoError => {
                // parse question
                let mut offset: usize = 12;
                let mut questions: Vec<Question> = Vec::new();
                for _ in 0..header.qd_count {
                    let (question, size) = Question::parse(&buffer, offset);
                    questions.push(question);
                    offset += size;
                }

                // parse resource record
                let mut answers: Vec<ResourceRecord> = Vec::new();
                for _ in 0..header.an_count {
                    let (answer, size) = ResourceRecord::parse(buffer, offset);
                    answers.push(answer);
                    offset += size;
                }

                Message {
                    header,
                    questions,
                    answers,

                    bypass_gfw: false,
                }
            }
            _ => {
                // on error
                Message {
                    header,
                    questions: Vec::new(),
                    answers: Vec::new(),

                    bypass_gfw: false,
                }
            }
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Message::new(Header::default(), true)
    }
}

impl Into<BytesMut> for Message {
    fn into(self) -> BytesMut {
        let mut message = self.header.into();

        // bypass gfw
        if self.header.qd_count > 1 && self.bypass_gfw {
            self.questions[0].append_gfw(&mut message);
        }

        // append question
        for q in self.questions.iter() {
            q.append_to(&mut message);
        }

        // TODO: append a to message
        // for a in self.answers.iter() {}

        message
    }
}

impl Message {
    pub fn new(header: Header, bypass_gfw: bool) -> Message {
        let mut message = Message {
            header,
            questions: Vec::new(),
            answers: Vec::new(),
            bypass_gfw,
        };
        message.header.qd_count = if bypass_gfw { 1 } else { 0 };
        message
    }

    pub fn add_question(&mut self, q: Question) {
        self.questions.push(q);
        self.header.qd_count += 1;
    }

    pub fn bypass_gfw(&mut self, bypass_gfw: bool) {
        if self.bypass_gfw && !bypass_gfw {
            self.header.qd_count -= 1;
        } else if !self.bypass_gfw && bypass_gfw {
            self.header.qd_count += 1;
        }
        self.bypass_gfw = bypass_gfw;
    }
}
