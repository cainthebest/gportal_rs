///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApibillingBillStatus {
    #[serde(rename = "UNPAID")]
    UNPAID,
    #[serde(rename = "PAID")]
    PAID,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "REFUNDED")]
    REFUNDED,
    #[serde(rename = "ERROR")]
    ERROR,
}
