#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProjectTrafficUsage {
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "bytesSent", skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<String>,
}

impl ProjectProjectTrafficUsage {
    pub fn new() -> ProjectProjectTrafficUsage {
        ProjectProjectTrafficUsage {
            ip: None,
            bytes_sent: None,
        }
    }
}
