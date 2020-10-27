use super::client::*;
use glow_utils::binary::*;
use neli::err::NlError;

#[derive(Debug)]
pub struct RegData {
    pub country: String,

    pub rules: Vec<RegRule>,

    pub phy_id: Option<u32>,
    pub self_managed: bool,
    pub dfs_domain: Option<u8>,
}

#[derive(Debug)]
pub struct RegRule {
    flags: u32,
    start_freq_khz: u32,
    end_freq_khz: u32,
    max_bw_khz: u32,
    max_ant_gain_mbi: u32,
    max_eirp_mbm: u32,

    dfs_cac_time: Option<u32>,
}

impl Default for RegData {
    fn default() -> Self {
        RegData {
            country: String::new(),
            rules: Vec::new(),
            phy_id: None,
            self_managed: false,
            dfs_domain: None,
        }
    }
}

impl Default for RegRule {
    fn default() -> Self {
        RegRule {
            flags: 0,
            start_freq_khz: 0,
            end_freq_khz: 0,
            max_bw_khz: 0,
            max_ant_gain_mbi: 0,
            max_eirp_mbm: 0,
            dfs_cac_time: None,
        }
    }
}

impl NL80211Client {
    // COMMAND(reg, get, NULL, NL80211_CMD_GET_REG, 0, CIB_NONE, handle_reg_get,
    //   "Print out the kernel's current regulatory domain information.");
    pub fn reg_get(&mut self, phy: Option<&str>) -> Result<RegData, NlError> {
        match phy {
            Some(phy) => {
                // TODO
                Ok(RegData::default())
            }
            None => self.reg_dump(),
        }
    }

    // HIDDEN(reg, dump, NULL, NL80211_CMD_GET_REG, NLM_F_DUMP, CIB_NONE, handle_reg_dump);
    pub fn reg_dump(&mut self) -> Result<RegData, NlError> {
        let mut result = RegData::default();
        let mut socket = self.send(Nl80211Cmd::CmdGetReg)?;

        use super::attr::*;
        use super::cmd::Nl80211Cmd;
        use neli::genl::Genlmsghdr;

        let iter = socket.iter::<Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(false);
        for response_result in iter {
            let msg = response_result?;
            let handle = msg.get_payload()?.get_attr_handle();
            for attr in handle.iter() {
                let payload = attr.payload.as_ref();
                match &attr.nla_type {
                    Nl80211Attr::AttrRegAlpha2 => {
                        result.country.push(payload[0].into());
                        result.country.push(payload[1].into());
                    }
                    Nl80211Attr::AttrRegRules => {
                        let handle = attr.get_attr_handle::<NlaNested>()?;
                        for attr in handle.iter() {
                            let mut rule = RegRule::default();
                            let nested = attr.get_attr_handle()?;
                            for attr in nested.iter() {
                                let payload = attr.payload.as_ref();
                                match &attr.nla_type {
                                    Nl80211RegRuleAttr::AttrRegRuleFlags => {
                                        rule.flags = parse_u32(payload, attr.nla_network_order);
                                    }
                                    Nl80211RegRuleAttr::AttrFreqRangeStart => {
                                        rule.start_freq_khz =
                                            parse_u32(payload, attr.nla_network_order);
                                    }
                                    Nl80211RegRuleAttr::AttrFreqRangeEnd => {
                                        rule.end_freq_khz =
                                            parse_u32(payload, attr.nla_network_order);
                                    }
                                    Nl80211RegRuleAttr::AttrFreqRangeMaxBw => {
                                        rule.max_bw_khz =
                                            parse_u32(payload, attr.nla_network_order);
                                    }
                                    Nl80211RegRuleAttr::AttrPowerRuleMaxAntGain => {
                                        rule.max_ant_gain_mbi =
                                            parse_u32(payload, attr.nla_network_order);
                                    }
                                    Nl80211RegRuleAttr::AttrPowerRuleMaxEirp => {
                                        rule.max_eirp_mbm =
                                            parse_u32(payload, attr.nla_network_order);
                                    }
                                    Nl80211RegRuleAttr::AttrDfsCacTime => {
                                        rule.dfs_cac_time =
                                            parse_some_u32(payload, attr.nla_network_order);
                                    }
                                    _ => {}
                                }
                            }
                            result.rules.push(rule);
                        }
                    }
                    Nl80211Attr::AttrWiphy => {
                        glow_utils::binary::parse_some_u32(payload, attr.nla_network_order);
                    }
                    Nl80211Attr::AttrWiphySelfManagedReg => {
                        result.self_managed = true;
                    }
                    Nl80211Attr::AttrDfsRegion => {
                        result.dfs_domain = Some(payload[0]);
                    }
                    _ => (),
                }
            }

            break;
        }

        Ok(result)
    }

    // COMMAND(reg, set, "<ISO/IEC 3166-1 alpha2>",
    //  NL80211_CMD_REQ_SET_REG, 0, CIB_NONE, handle_reg_set,
    // "Notify the kernel about the current regulatory domain.");
    pub fn reg_set(&self, country: &str) {
        // TODO
    }

    // COMMAND(reg, reload, NULL, NL80211_CMD_RELOAD_REGDB, 0, CIB_NONE,
    //   handle_reg_reload, "Reload the kernel's regulatory database.");
    pub fn reg_reload(&self) {
        // TODO
    }
}
