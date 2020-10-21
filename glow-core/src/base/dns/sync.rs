use super::common::*;
use super::types::QType;
use std::net::UdpSocket;
use std::str;

pub struct DNSClient {
    addr: String,
    upstream: String,
}

impl DNSClient {
    pub fn new(addr: &str) -> DNSClient {
        DNSClient {
            addr: addr.to_owned(),
            upstream: String::new(),
        }
    }

    pub fn upstream(&mut self, addr: &str) -> &DNSClient {
        self.upstream = addr.to_owned();
        self
    }

    pub fn query(
        &self,
        domain: &str,
        qtype: QType,
        bypass_gfw: bool,
    ) -> std::io::Result<(Header, Vec<Question>, Vec<ResourceRecord>)> {
        let mut qd_count = 1;
        if bypass_gfw {
            qd_count += 1;
        }

        let mut header =
            Header::new(0xff, HeaderFlag::DEFAULT_QUERY_FLAG, qd_count, 0, 0, 0).into();

        if bypass_gfw {
            Question::new(domain, qtype).append_gfw(&mut header);
        }
        Question::new(domain, qtype).append_to(&mut header);

        // send query
        let socket = UdpSocket::bind(&self.addr)?;
        socket.connect(&self.upstream)?;
        socket.send(&header)?;

        // receive response
        let mut buffer = [0u8; 1500];
        socket.recv_from(&mut buffer)?;

        // parse header
        let header = Header::from(&buffer[..]);

        let flag = header.flag();
        match flag.rcode {
            FlagRCode::NoError => {}
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
            }
        }

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
            let (answer, size) = ResourceRecord::parse(&buffer, offset);
            answers.push(answer);
            offset += size;
        }

        Ok((header, questions, answers))
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use super::*;
    #[test]
    fn test_query_a() {
        let mut client = super::DNSClient::new("0.0.0.0:9876");
        client.upstream("1.0.0.1:53");
        let (_, _, answers) = client
            .query("glow.mmf.moe", QType::Type(Type::A), true)
            .unwrap();
        assert_eq!(answers.len(), 1);

        match answers[0].r_data {
            RData::A(addr) => {
                assert_eq!("1.0.0.1".parse(), Ok(addr));
                return;
            }
            _ => {}
        }
        panic!("No AAAA record found!");
    }

    #[test]
    fn test_query_aaaa() {
        let mut client = super::DNSClient::new("0.0.0.0:9877");
        client.upstream("1.0.0.1:53");
        let (_, _, answers) = client
            .query("glow.mmf.moe", QType::Type(Type::AAAA), true)
            .unwrap();
        assert_eq!(answers.len(), 1);

        match answers[0].r_data {
            RData::AAAA(addr) => {
                assert_eq!("2606:4700:20::ac43:45a9".parse(), Ok(addr));
                return;
            }
            _ => {}
        }
        panic!("No AAAA record found!");
    }
}
