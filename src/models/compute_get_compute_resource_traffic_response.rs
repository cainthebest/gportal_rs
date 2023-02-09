#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeGetComputeResourceTrafficResponse {
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<crate::models::ComputeTrafficUsage>>,
}

impl ComputeGetComputeResourceTrafficResponse {
    pub fn new() -> ComputeGetComputeResourceTrafficResponse {
        ComputeGetComputeResourceTrafficResponse { usage: None }
    }
}
