use futures::stream::TryStreamExt;
use glow_utils::binary::EasyMerge;
use rtnetlink::packet::nlas::address::Nla;
use rtnetlink::packet::AddressMessage;
use rtnetlink::{new_connection, Error};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[tokio::main]
pub async fn ip_get() -> Result<(), Error> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let mut links = handle.link().get().execute();
    while let Some(link) = links.try_next().await? {
        let mut addr_count = 0;
        let mut addr = handle
            .address()
            .get()
            .set_link_index_filter(link.header.index)
            .execute();
        while let Some(msg) = addr.try_next().await? {
            let msg: AddressMessage = msg;

            for nla in msg.nlas.iter() {
                if let Some(addr) = match nla {
                    Nla::Address(addr) => {
                        if addr.len() == 4 {
                            // IPv4
                            Some(IpAddr::V4(Ipv4Addr::new(
                                addr[0], addr[1], addr[2], addr[3],
                            )))
                        } else if addr.len() == 16 {
                            // IPv6
                            Some(IpAddr::V6(Ipv6Addr::new(
                                addr[0].merge_lower(addr[1]),
                                addr[2].merge_lower(addr[3]),
                                addr[4].merge_lower(addr[5]),
                                addr[6].merge_lower(addr[7]),
                                addr[8].merge_lower(addr[9]),
                                addr[10].merge_lower(addr[11]),
                                addr[12].merge_lower(addr[13]),
                                addr[14].merge_lower(addr[15]),
                            )))
                        // Some(IpAddr::V6(Ipv6Addr::from([
                        //     addr[0], addr[1], addr[2], addr[3], addr[4], addr[5], addr[6],
                        //     addr[7], addr[8], addr[9], addr[10], addr[11], addr[12], addr[13],
                        //     addr[14], addr[15],
                        // ]
                        //     as [u8; 16])))
                        } else {
                            None
                        }
                    }
                    Nla::Label(l) => {
                        println!("{}", l);
                        None
                    }
                    _ => {
                        println!("{:?}", nla);
                        None
                    }
                } {
                    addr_count += 1;
                    println!("{}", addr.to_string());
                }
            }
        }

        if addr_count > 0 {
            println!("------");
        }
    }

    Ok(())
}
