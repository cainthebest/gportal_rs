///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComputeComputeResourceRenewState {
    #[serde(rename = "COMPUTE_RESOURCE_RENEW_STATE_EXTEND")]
    EXTEND,
    #[serde(rename = "COMPUTE_RESOURCE_RENEW_STATE_DELETE")]
    DELETE,
    #[serde(rename = "COMPUTE_RESOURCE_RENEW_STATE_TO_HOURLY")]
    TOHOURLY,
}
