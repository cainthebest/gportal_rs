///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkNetworkType {
    #[serde(rename = "MANAGEMENT")]
    MANAGEMENT,
    #[serde(rename = "PROVISIONING_POOL")]
    PROVISIONINGPOOL,
    #[serde(rename = "PUBLIC_POOL")]
    PUBLICPOOL,
    #[serde(rename = "PROJECT_PUBLIC")]
    PROJECTPUBLIC,
    #[serde(rename = "PROJECT_PRIVATE")]
    PROJECTPRIVATE,
}
