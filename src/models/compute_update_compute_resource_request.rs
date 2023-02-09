#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeUpdateComputeResourceRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl ComputeUpdateComputeResourceRequest {
    pub fn new() -> ComputeUpdateComputeResourceRequest {
        ComputeUpdateComputeResourceRequest {
            project_id: None,
            resource_id: None,
            fqdn: None,
            tags: None,
        }
    }
}
