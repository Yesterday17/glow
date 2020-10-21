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

    pub fn query(&self, domain: &str, qtype: QType, bypass_gfw: bool) -> std::io::Result<()> {
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
        println!("{:?}", header);

        let flag = header.flag();
        println!("{:?}", flag);
        match flag.rcode {
            FlagRCode::NoError => {}
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, ""));
            }
        }

        // parse question
        let mut offset: usize = 12;
        for _ in 0..header.qd_count {
            let (question, size) = Question::parse(&buffer, offset);
            offset += size;
            println!("{:?}", question);
        }

        // parse resource record
        for _ in 0..header.an_count {
            let (answer, size) = ResourceRecord::parse(&buffer, offset);
            offset += size;
            println!("{:?}", answer)
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::*;
    #[test]
    fn test_query_a() {
        let mut client = super::DNSClient::new("0.0.0.0:9876");
        client.upstream("1.0.0.1:53");
        client
            .query("facebook.com", QType::Type(Type::A), true)
            .unwrap();
        // TODO: assert_eq!(...)
    }
}
