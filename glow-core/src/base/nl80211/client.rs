use super::{attr::Nl80211Attr, constant::*, utils::phy_lookup};
use neli::{
    consts::nl::*,
    consts::genl::*,
    consts::socket::*,
    err::NlError,
    genl::{Genlmsghdr, Nlattr},
    nl::{NlPayload, Nlmsghdr},
    socket::*,
    types::*,
};
use std::str::FromStr;

pub enum CommandIdBy {
    Phy(u32),
    NetDev(u32),
    WDev(u32),
    None,
}

pub struct NL80211Client {
    family_id: u16,
    devidx: CommandIdBy,
}

impl NL80211Client {
    pub fn new() -> Result<NL80211Client, NlError> {
        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, &[])?;
        let family_id = socket.resolve_genl_family(NL80211_FAMILY_NAME)?;
        Ok(NL80211Client {
            family_id,
            devidx: CommandIdBy::None,
        })
    }

    pub fn set_phy(&mut self, phy: String) -> Result<(), NlError> {
        let phy_id = phy_lookup(phy)?;
        self.devidx = CommandIdBy::Phy(phy_id);
        Ok(())
    }

    pub fn set_netdev(&mut self, dev: String) -> Result<(), NlError> {
        unimplemented!();
    }

    pub fn set_wdev(&mut self, wdev: String) -> Result<(), NlError> {
        let wdev_id = u32::from_str(&wdev).map_err(NlError::new)?;
        self.devidx = CommandIdBy::WDev(wdev_id);
        Ok(())
    }

    pub(crate) fn send<T>(
        &mut self,
        cmd: T,
        flags: Option<NlmFFlags>,
        attrs: Option<GenlBuffer<Nl80211Attr, Buffer>>,
    ) -> Result<NlSocketHandle, NlError>
        where
            T: Cmd + std::fmt::Debug,
    {
        let mut attrs = GenlBuffer::from(match attrs {
            Some(a) => a,
            None => GenlBuffer::new(),
        });
        match self.devidx {
            CommandIdBy::Phy(id) => {
                let attr = Nlattr::new(
                    None,
                    false,
                    false,
                    super::attr::Nl80211Attr::AttrWiphy,
                    NlPayload::Payload(id),
                )?;
                attrs.push(attr);
            }
            CommandIdBy::NetDev(id) => {
                let attr = Nlattr::new(
                    None,
                    false,
                    false,
                    super::attr::Nl80211Attr::AttrIfindex,
                    NlPayload::Payload(id),
                )?;
                attrs.push(attr);
            }
            CommandIdBy::WDev(id) => {
                let attr = Nlattr::new(
                    None,
                    false,
                    false,
                    super::attr::Nl80211Attr::AttrWdev,
                    NlPayload::Payload(id),
                )?;
                attrs.push(attr);
            }
            _ => (),
        }
        let genlhdr = Genlmsghdr::new(cmd, 1, attrs);
        let msg = Nlmsghdr::new(
            None,
            self.family_id,
            match flags {
                Some(flags) => flags,
                None => NlmFFlags::new(&[NlmF::Request, NlmF::Ack]),
            },
            None,
            None,
            NlPayload::Payload(genlhdr),
        );

        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, &[])?;
        socket.send(msg)?;
        Ok(socket)
    }
}
