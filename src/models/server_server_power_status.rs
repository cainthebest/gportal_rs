///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServerServerPowerStatus {
    #[serde(rename = "SHUT_ON")]
    SHUTON,
    #[serde(rename = "SHUT_OFF")]
    SHUTOFF,
    #[serde(rename = "RESET")]
    RESET,
    #[serde(rename = "RESET_PLATFORM_MANAGEMENT")]
    RESETPLATFORMMANAGEMENT,
}
