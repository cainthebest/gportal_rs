#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentBillingAddress {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "vatId", skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "postcode", skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<i32>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl PaymentBillingAddress {
    pub fn new() -> PaymentBillingAddress {
        PaymentBillingAddress {
            id: None,
            company: None,
            name: None,
            vat_id: None,
            country_code: None,
            state: None,
            street: None,
            city: None,
            postcode: None,
            tax: None,
            created_at: None,
            updated_at: None,
            email: None,
        }
    }
}
