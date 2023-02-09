#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeTrafficUsageEntry {
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "bytesReceived", skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<String>,
    #[serde(rename = "packetsReceived", skip_serializing_if = "Option::is_none")]
    pub packets_received: Option<String>,
    #[serde(rename = "bytesSent", skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<String>,
    #[serde(rename = "packetsSent", skip_serializing_if = "Option::is_none")]
    pub packets_sent: Option<String>,
}

impl ComputeTrafficUsageEntry {
    pub fn new() -> ComputeTrafficUsageEntry {
        ComputeTrafficUsageEntry {
            timestamp: None,
            bytes_received: None,
            packets_received: None,
            bytes_sent: None,
            packets_sent: None,
        }
    }
}
