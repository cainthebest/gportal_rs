#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerMonitoringStatus {
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "online", skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}

impl ServerMonitoringStatus {
    pub fn new() -> ServerMonitoringStatus {
        ServerMonitoringStatus {
            ip: None,
            online: None,
        }
    }
}
