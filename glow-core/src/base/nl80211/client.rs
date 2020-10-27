use super::cmd::Nl80211Cmd;
use super::constant::*;
use crate::genlmsg_put;
use neli::consts::Cmd;
use neli::consts::NlFamily;
use neli::err::*;
use neli::socket::*;
use neli::utils::U32Bitmask;

pub struct NL80211Client {
    family_id: u16,
}

impl NL80211Client {
    pub fn new() -> Result<NL80211Client, NlError> {
        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, U32Bitmask::empty())?;
        let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;
        Ok(NL80211Client { family_id })
    }

    pub fn send<T>(&mut self, cmd: T) -> Result<NlSocketHandle, NlError>
    where
        T: Cmd + std::fmt::Debug,
    {
        genlmsg_put!(
            msg,
            0,
            0,
            self.family_id,
            0,
            [NlmF::Request, NlmF::Ack],
            cmd,
            1
        );

        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, U32Bitmask::empty())?;
        socket.send(msg)?;
        Ok(socket)
    }
}

#[macro_export]
macro_rules! nl_response {
    ($self: ident, $resp: ident, $handle: ident,$success: block, $fail: block) => {
        use super::attr::*;
        use super::cmd::Nl80211Cmd;
        use neli::consts::Nlmsg;
        use neli::genl::*;

        let socket = &mut $self.socket;
        let mut iter = socket.iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>();
        while let Some(Ok($resp)) = iter.next() {
            match $resp.nl_type {
                Nlmsg::Error => $fail,
                Nlmsg::Done => break,
                _ => {
                    let $handle = $resp.nl_payload.get_attr_handle();
                    $success;
                }
            };
        }
    };

    ($msg: ident, $self: ident) => {
        use super::attr::*;
        use super::cmd::Nl80211Cmd;
        use neli::consts::Nlmsg;
        use neli::genl::*;

        let socket = &mut $self.socket;
        let $msg = socket
            .recv::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(None)
            .unwrap();
    };
}

// genlmsg_put(msg, 0,    0,   id,     0,      flags, cmd, 0      )
// genlmsg_put(msg, port, seq, family, hdrlen, flags, cmd, version)
//  1. nlmsg_put(msg, port, seq, family, GENL_HDRLEN + hdrlen, flags)
//  2. set cmd and vesion
#[macro_export]
macro_rules! genlmsg_put {
    ($msg: ident, $port: expr, $seq: expr, $family: expr, $hdrlen: expr, $flags: expr, $cmd: expr, $version: expr) => {
        use neli::consts::*;
        use neli::genl::Genlmsghdr;
        use neli::nl::NlPayload;
        use neli::nl::Nlmsghdr;
        use neli::types::*;

        let attrs = GenlBuffer::<NlAttrTypeWrapper, Buffer>::new();
        let genlhdr = Genlmsghdr::new($cmd, $version, attrs);
        let $msg = Nlmsghdr::new(
            if $hdrlen != 0 { Some($hdrlen) } else { None },
            $family,
            NlmFFlags::new(&$flags),
            if $seq != 0 { Some($seq) } else { None },
            if $port != 0 { Some($port) } else { None },
            NlPayload::Payload(genlhdr),
        );
    };
}

mod test2 {
    #[test]
    fn test() {
        // let phy = String::from("");
        let reg = super::NL80211Client::new().unwrap().reg_get(None).unwrap();
        println!("{:#?}", reg);
    }
}
