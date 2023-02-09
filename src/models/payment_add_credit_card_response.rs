#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentAddCreditCardResponse {
    #[serde(rename = "creditCard", skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<crate::models::PaymentCreditCard>,
}

impl PaymentAddCreditCardResponse {
    pub fn new() -> PaymentAddCreditCardResponse {
        PaymentAddCreditCardResponse { credit_card: None }
    }
}
