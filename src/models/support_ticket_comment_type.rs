///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportTicketCommentType {
    #[serde(rename = "CUSTOMER")]
    CUSTOMER,
    #[serde(rename = "AGENT")]
    AGENT,
}
