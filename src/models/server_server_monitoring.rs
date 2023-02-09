#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerServerMonitoring {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<crate::models::ServerMonitoringTag>,
}

impl ServerServerMonitoring {
    pub fn new() -> ServerServerMonitoring {
        ServerServerMonitoring {
            id: None,
            ip: None,
            tag: None,
        }
    }
}
