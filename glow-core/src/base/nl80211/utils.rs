use glow_utils::fs::read_file;
use neli::err::NlError;
use std::str::FromStr;

pub fn phy_lookup(phy: String) -> Result<u32, NlError> {
    let str = read_file(&format!("/sys/class/ieee80211/{}/index", phy)).map_err(NlError::new)?;
    u32::from_str(str.trim()).map_err(NlError::new)
}

pub fn is_alpha2(str: &str) -> bool {
    str.len() == 2 && is_alpha_upper(str.as_bytes()[0]) && is_alpha_upper(str.as_bytes()[1])
}

pub fn is_alpha_upper(c: u8) -> bool {
    c >= 65 && c <= 90
}

pub fn is_world_regdom(str: &str) -> bool {
    str.len() == 2 && str.as_bytes()[0] == 48 && str.as_bytes()[1] == 48
}
