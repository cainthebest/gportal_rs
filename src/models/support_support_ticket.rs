#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportSupportTicket {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<crate::models::SupportSupportTicketPriority>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::SupportTicketStatus>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::BasicBasicUser>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<crate::models::SupportSupportTicketComment>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl SupportSupportTicket {
    pub fn new() -> SupportSupportTicket {
        SupportSupportTicket {
            id: None,
            title: None,
            priority: None,
            status: None,
            user: None,
            number: None,
            comments: None,
            created_at: None,
            updated_at: None,
        }
    }
}
