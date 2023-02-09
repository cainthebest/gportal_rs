#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportAddProjectSupportTicketCommentRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl SupportAddProjectSupportTicketCommentRequest {
    pub fn new() -> SupportAddProjectSupportTicketCommentRequest {
        SupportAddProjectSupportTicketCommentRequest {
            project_id: None,
            id: None,
            comment: None,
        }
    }
}
