#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerListMonitoringTargetsResponse {
    #[serde(rename = "monitoring", skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<Vec<crate::models::ServerServerMonitoring>>,
}

impl ServerListMonitoringTargetsResponse {
    pub fn new() -> ServerListMonitoringTargetsResponse {
        ServerListMonitoringTargetsResponse { monitoring: None }
    }
}
