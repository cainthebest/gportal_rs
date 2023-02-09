#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportListProjectSupportTicketsResponse {
    #[serde(rename = "tickets", skip_serializing_if = "Option::is_none")]
    pub tickets: Option<Vec<crate::models::SupportSupportTicket>>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl SupportListProjectSupportTicketsResponse {
    pub fn new() -> SupportListProjectSupportTicketsResponse {
        SupportListProjectSupportTicketsResponse {
            tickets: None,
            total_count: None,
        }
    }
}
