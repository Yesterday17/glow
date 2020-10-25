extern crate neli;

use super::attr::*;
use super::cmd::Nl80211Cmd;
use neli::consts::{NlFamily, NlmF, Nlmsg};
use neli::err::*;
use neli::genl::*;
use neli::nl::Nlmsghdr;
use neli::nlattr::*;
use neli::socket::*;

// genlmsg_put(msg, 0,    0,   id,     0,      flags, cmd, 0      )
// genlmsg_put(msg, port, seq, family, hdrlen, flags, cmd, version)
//  1. nlmsg_put(msg, port, seq, family, GENL_HDRLEN + hdrlen, flags)
//  2. set cmd and vesion

pub fn connect() -> Result<NlSocket, NlError> {
    let mut socket = NlSocket::connect(NlFamily::Generic, None, None, true).unwrap();
    let id = socket
        .resolve_genl_family(super::constant::NL80211_FAMILY_NAME)
        .unwrap();

    let attrs: Vec<Nlattr<Nl80211Attr, Vec<u8>>> = vec![];
    let genlhdr = Genlmsghdr::new(Nl80211Cmd::CmdGetReg, 0, attrs)?;
    let nlhdr = Nlmsghdr::new(None, id, vec![NlmF::Request], None, None, genlhdr);
    socket.send_nl(nlhdr)?;

    let mut iter = socket.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();
    while let Some(Ok(response)) = iter.next() {
        match response.nl_type {
            Nlmsg::Error => panic!("Error"),
            Nlmsg::Done => break,
            _ => {
                let handle = response.nl_payload.get_attr_handle();
                // TODO
            }
        };
    }

    Ok(socket)
}
