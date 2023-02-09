///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComputeComputeResourceStatus {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "INSTALLING")]
    INSTALLING,
    #[serde(rename = "POST_INSTALLING")]
    POSTINSTALLING,
    #[serde(rename = "READY")]
    READY,
    #[serde(rename = "ONLINE")]
    ONLINE,
    #[serde(rename = "RESCUE_MODE")]
    RESCUEMODE,
}
