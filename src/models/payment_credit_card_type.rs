///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentCreditCardType {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "MASTERCARD")]
    MASTERCARD,
    #[serde(rename = "VISA")]
    VISA,
    #[serde(rename = "AMEX")]
    AMEX,
    #[serde(rename = "DISCOVER")]
    DISCOVER,
    #[serde(rename = "DINERS")]
    DINERS,
    #[serde(rename = "JCB")]
    JCB,
}
