#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportSupportTicketSla {
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<crate::models::SupportSupportTicketPriority>,
    #[serde(
        rename = "firstReplyTimeHours",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_reply_time_hours: Option<i32>,
    #[serde(rename = "nextReplyTimeHours", skip_serializing_if = "Option::is_none")]
    pub next_reply_time_hours: Option<i32>,
}

impl SupportSupportTicketSla {
    pub fn new() -> SupportSupportTicketSla {
        SupportSupportTicketSla {
            priority: None,
            first_reply_time_hours: None,
            next_reply_time_hours: None,
        }
    }
}
