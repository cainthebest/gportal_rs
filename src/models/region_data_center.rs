#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionDataCenter {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "short", skip_serializing_if = "Option::is_none")]
    pub short: Option<String>,
    #[serde(rename = "serverPrefix", skip_serializing_if = "Option::is_none")]
    pub server_prefix: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<crate::models::RegionRegion>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::RegionDataCenterStatus>,
    #[serde(rename = "latencyEndpoint", skip_serializing_if = "Option::is_none")]
    pub latency_endpoint: Option<String>,
}

impl RegionDataCenter {
    pub fn new() -> RegionDataCenter {
        RegionDataCenter {
            id: None,
            name: None,
            short: None,
            server_prefix: None,
            region: None,
            status: None,
            latency_endpoint: None,
        }
    }
}
