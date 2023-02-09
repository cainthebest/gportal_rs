#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportCreateProjectSupportTicketRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<crate::models::SupportSupportTicketPriority>,
}

impl SupportCreateProjectSupportTicketRequest {
    pub fn new() -> SupportCreateProjectSupportTicketRequest {
        SupportCreateProjectSupportTicketRequest {
            project_id: None,
            title: None,
            message: None,
            priority: None,
        }
    }
}
