///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentPaymentMethod {
    #[serde(rename = "CREDIT_CARD")]
    CREDITCARD,
    #[serde(rename = "BANK_TRANSFER")]
    BANKTRANSFER,
}
