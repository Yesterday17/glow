use glow_utils::fs::read_file;
use neli::err::NlError;
use std::str::FromStr;

pub fn phy_lookup(phy: String) -> Result<u32, NlError> {
    let str = read_file(&format!("/sys/class/ieee80211/{}/index", phy)).map_err(NlError::new)?;
    u32::from_str(str.trim()).map_err(NlError::new)
}
