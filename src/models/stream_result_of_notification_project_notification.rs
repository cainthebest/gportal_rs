#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamResultOfNotificationProjectNotification {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::models::NotificationProjectNotification>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::models::RpcStatus>,
}

impl StreamResultOfNotificationProjectNotification {
    pub fn new() -> StreamResultOfNotificationProjectNotification {
        StreamResultOfNotificationProjectNotification {
            result: None,
            error: None,
        }
    }
}
