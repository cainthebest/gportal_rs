///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiPlatform {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "WINDOWS")]
    WINDOWS,
    #[serde(rename = "MAC_OS")]
    MACOS,
    #[serde(rename = "LINUX")]
    LINUX,
    #[serde(rename = "ANDROID")]
    ANDROID,
    #[serde(rename = "IOS")]
    IOS,
}
