#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportCloseProjectSupportTicketRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl SupportCloseProjectSupportTicketRequest {
    pub fn new() -> SupportCloseProjectSupportTicketRequest {
        SupportCloseProjectSupportTicketRequest {
            project_id: None,
            id: None,
        }
    }
}
