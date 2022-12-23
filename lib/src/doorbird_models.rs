use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    #[serde(rename = "BHA")]
    pub bha: BHAInfo,
}

impl std::fmt::Display for InfoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BHA='{:?}'", self.bha)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BHAInfo {
    #[serde(rename = "RETURNCODE")]
    pub return_code: String,
    #[serde(rename = "VERSION")]
    pub version: Vec<Version>,
}

impl std::fmt::Display for BHAInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RETURNCODE='{:?}'", self.return_code)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "FIRMWARE")]
    pub firmware: String,
    #[serde(rename = "BUILD_NUMBER")]
    pub build_number: String,
    #[serde(rename = "WIFI_MAC_ADDR")]
    pub wifi_mac_addr: String,
    #[serde(rename = "RELAYS")]
    pub relays: Vec<String>,
    #[serde(rename = "DEVICE-TYPE")]
    pub device_type: String,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FIRMWARE='{:?}', BUILD_NUMBER='{:?}', WIFI_MAC_ADDR='{:?}', DEVICETYPE='{:?}'",
            self.firmware, self.build_number, self.wifi_mac_addr, self.device_type
        )
    }
}
