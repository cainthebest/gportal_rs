///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportTicketStatus {
    #[serde(rename = "OPEN")]
    OPEN,
    #[serde(rename = "IN_PROGRESS")]
    INPROGRESS,
    #[serde(rename = "CLOSED")]
    CLOSED,
}
