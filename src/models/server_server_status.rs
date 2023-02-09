///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServerServerStatus {
    #[serde(rename = "SETUP_PLATFORM_MANAGEMENT")]
    SETUPPLATFORMMANAGEMENT,
    #[serde(rename = "AVAILABLE")]
    AVAILABLE,
    #[serde(rename = "PROVISIONING")]
    PROVISIONING,
    #[serde(rename = "POST_PROVISIONING")]
    POSTPROVISIONING,
    #[serde(rename = "PROVISIONED")]
    PROVISIONED,
    #[serde(rename = "DEPROVISIONING")]
    DEPROVISIONING,
    #[serde(rename = "DELETING")]
    DELETING,
    #[serde(rename = "IMPORTING")]
    IMPORTING,
}
