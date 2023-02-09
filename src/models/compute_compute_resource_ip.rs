#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeComputeResourceIp {
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "mac", skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::NetworkNetworkType>,
}

impl ComputeComputeResourceIp {
    pub fn new() -> ComputeComputeResourceIp {
        ComputeComputeResourceIp {
            ip: None,
            mac: None,
            _type: None,
        }
    }
}
