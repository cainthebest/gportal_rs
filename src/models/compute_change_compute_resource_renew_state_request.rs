#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeChangeComputeResourceRenewStateRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "renewState", skip_serializing_if = "Option::is_none")]
    pub renew_state: Option<crate::models::ComputeComputeResourceRenewState>,
}

impl ComputeChangeComputeResourceRenewStateRequest {
    pub fn new() -> ComputeChangeComputeResourceRenewStateRequest {
        ComputeChangeComputeResourceRenewStateRequest {
            project_id: None,
            resource_id: None,
            renew_state: None,
        }
    }
}
