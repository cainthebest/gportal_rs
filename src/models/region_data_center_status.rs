///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegionDataCenterStatus {
    #[serde(rename = "AVAILABLE")]
    AVAILABLE,
    #[serde(rename = "COMING_SOON")]
    COMINGSOON,
    #[serde(rename = "DISABLED")]
    DISABLED,
}
