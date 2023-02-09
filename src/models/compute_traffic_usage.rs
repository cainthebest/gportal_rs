#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeTrafficUsage {
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "traffic", skip_serializing_if = "Option::is_none")]
    pub traffic: Option<Vec<crate::models::ComputeTrafficUsageEntry>>,
}

impl ComputeTrafficUsage {
    pub fn new() -> ComputeTrafficUsage {
        ComputeTrafficUsage {
            ip: None,
            traffic: None,
        }
    }
}
