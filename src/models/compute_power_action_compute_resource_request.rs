#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputePowerActionComputeResourceRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ServerServerPowerStatus>,
}

impl ComputePowerActionComputeResourceRequest {
    pub fn new() -> ComputePowerActionComputeResourceRequest {
        ComputePowerActionComputeResourceRequest {
            project_id: None,
            resource_id: None,
            status: None,
        }
    }
}
