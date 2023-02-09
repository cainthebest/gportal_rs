///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageImageType {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "QCOW2")]
    QCOW2,
    #[serde(rename = "RAW")]
    RAW,
}
