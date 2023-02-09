#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentListCreditCardsResponse {
    #[serde(rename = "creditCards", skip_serializing_if = "Option::is_none")]
    pub credit_cards: Option<Vec<crate::models::PaymentCreditCard>>,
}

impl PaymentListCreditCardsResponse {
    pub fn new() -> PaymentListCreditCardsResponse {
        PaymentListCreditCardsResponse { credit_cards: None }
    }
}
