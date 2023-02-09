#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentDeleteCreditCardRequest {
    #[serde(rename = "cardId", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

impl PaymentDeleteCreditCardRequest {
    pub fn new() -> PaymentDeleteCreditCardRequest {
        PaymentDeleteCreditCardRequest { card_id: None }
    }
}
