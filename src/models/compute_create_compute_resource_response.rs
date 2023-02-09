#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeCreateComputeResourceResponse {
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<crate::models::ComputeComputeResource>>,
}

impl ComputeCreateComputeResourceResponse {
    pub fn new() -> ComputeCreateComputeResourceResponse {
        ComputeCreateComputeResourceResponse { resources: None }
    }
}
