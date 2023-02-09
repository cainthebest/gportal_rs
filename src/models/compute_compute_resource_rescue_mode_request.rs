#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeComputeResourceRescueModeRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "rescueMode", skip_serializing_if = "Option::is_none")]
    pub rescue_mode: Option<crate::models::ComputeRescueMode>,
}

impl ComputeComputeResourceRescueModeRequest {
    pub fn new() -> ComputeComputeResourceRescueModeRequest {
        ComputeComputeResourceRescueModeRequest {
            project_id: None,
            resource_id: None,
            rescue_mode: None,
        }
    }
}
