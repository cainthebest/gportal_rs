///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkServerInterfaceType {
    #[serde(rename = "PLATFORM_MANAGEMENT")]
    PLATFORMMANAGEMENT,
    #[serde(rename = "NETWORK")]
    NETWORK,
}
