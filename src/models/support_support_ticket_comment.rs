#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportSupportTicketComment {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::BasicBasicUser>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::SupportTicketCommentType>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl SupportSupportTicketComment {
    pub fn new() -> SupportSupportTicketComment {
        SupportSupportTicketComment {
            user: None,
            _type: None,
            comment: None,
            created_at: None,
        }
    }
}
