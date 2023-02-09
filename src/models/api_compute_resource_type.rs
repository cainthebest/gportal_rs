///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiComputeResourceType {
    #[serde(rename = "BARE_METAL")]
    BAREMETAL,
    #[serde(rename = "VIRTUAL")]
    _VIRTUAL,
}
