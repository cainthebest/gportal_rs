#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeListComputeResourcesResponse {
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<crate::models::ComputeComputeResource>>,
    #[serde(rename = "pagesTotal", skip_serializing_if = "Option::is_none")]
    pub pages_total: Option<i32>,
}

impl ComputeListComputeResourcesResponse {
    pub fn new() -> ComputeListComputeResourcesResponse {
        ComputeListComputeResourcesResponse {
            resources: None,
            pages_total: None,
        }
    }
}
