#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeDestroyComputeResourceRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl ComputeDestroyComputeResourceRequest {
    pub fn new() -> ComputeDestroyComputeResourceRequest {
        ComputeDestroyComputeResourceRequest {
            project_id: None,
            resource_id: None,
        }
    }
}
