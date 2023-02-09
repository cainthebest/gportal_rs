#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionListDataCenterResponse {
    #[serde(rename = "datacenters", skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<Vec<crate::models::RegionDataCenter>>,
}

impl RegionListDataCenterResponse {
    pub fn new() -> RegionListDataCenterResponse {
        RegionListDataCenterResponse { datacenters: None }
    }
}
