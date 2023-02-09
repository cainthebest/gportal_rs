///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServerMonitoringTag {
    #[serde(rename = "MONITOR_PLATFORM_MANAGEMENT")]
    PLATFORMMANAGEMENT,
    #[serde(rename = "MONITOR_HOST")]
    HOST,
}
