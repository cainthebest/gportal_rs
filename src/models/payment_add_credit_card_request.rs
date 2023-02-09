#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentAddCreditCardRequest {
    #[serde(rename = "holder", skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "expiryMonth", skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<String>,
    #[serde(rename = "expiryYear", skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<String>,
    #[serde(rename = "cvc", skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::PaymentCreditCardType>,
}

impl PaymentAddCreditCardRequest {
    pub fn new() -> PaymentAddCreditCardRequest {
        PaymentAddCreditCardRequest {
            holder: None,
            number: None,
            expiry_month: None,
            expiry_year: None,
            cvc: None,
            _type: None,
        }
    }
}
