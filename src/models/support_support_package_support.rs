#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportSupportPackageSupport {
    #[serde(rename = "ticket", skip_serializing_if = "Option::is_none")]
    pub ticket: Option<bool>,
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<bool>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<bool>,
    #[serde(rename = "ticketPriorities", skip_serializing_if = "Option::is_none")]
    pub ticket_priorities: Option<Vec<crate::models::SupportSupportTicketPriority>>,
    #[serde(rename = "ticketSlas", skip_serializing_if = "Option::is_none")]
    pub ticket_slas: Option<Vec<crate::models::SupportSupportTicketSla>>,
}

impl SupportSupportPackageSupport {
    pub fn new() -> SupportSupportPackageSupport {
        SupportSupportPackageSupport {
            ticket: None,
            chat: None,
            phone: None,
            ticket_priorities: None,
            ticket_slas: None,
        }
    }
}
