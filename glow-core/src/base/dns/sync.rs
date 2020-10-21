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

        let flag = HeaderFlagBuilder::new()
            .qr_query()
            .op_standard_query()
            .rd_on()
            .build();
        let mut header = Header::new(0xff, flag, qd_count, 0, 0, 0).to_raw();

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
        let header = Header::from(&buffer);
        println!("{:?}", header);

        // parse question
        let mut offset: usize = 12;
        for _ in 0..header.qd_count {
            let (question, size) = Question::parse(&buffer, offset);
            offset += size;
            println!("{:?}", question);
        }

        // TODO: parse resource record

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
