///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServerPlatformManagementType {
    #[serde(rename = "IDRAC")]
    IDRAC,
    #[serde(rename = "ILO5")]
    ILO5,
}
