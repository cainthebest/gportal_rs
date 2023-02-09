///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportSupportTicketPriority {
    #[serde(rename = "NORMAL")]
    NORMAL,
    #[serde(rename = "LOW")]
    LOW,
    #[serde(rename = "HIGH")]
    HIGH,
}
