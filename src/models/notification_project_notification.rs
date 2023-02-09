#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationProjectNotification {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::ProjectProject>,
    #[serde(rename = "computeResource", skip_serializing_if = "Option::is_none")]
    pub compute_resource: Option<crate::models::ComputeComputeResource>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::models::NetworkNetwork>,
}

impl NotificationProjectNotification {
    pub fn new() -> NotificationProjectNotification {
        NotificationProjectNotification {
            project: None,
            compute_resource: None,
            network: None,
        }
    }
}
