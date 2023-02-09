#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentCreditCard {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "holder", skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::PaymentCreditCardType>,
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl PaymentCreditCard {
    pub fn new() -> PaymentCreditCard {
        PaymentCreditCard {
            id: None,
            holder: None,
            number: None,
            _type: None,
            expired: None,
            created_at: None,
            updated_at: None,
            expires_at: None,
        }
    }
}
