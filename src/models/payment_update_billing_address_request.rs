#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentUpdateBillingAddressRequest {
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
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl PaymentUpdateBillingAddressRequest {
    pub fn new() -> PaymentUpdateBillingAddressRequest {
        PaymentUpdateBillingAddressRequest {
            id: None,
            company: None,
            name: None,
            vat_id: None,
            country_code: None,
            state: None,
            street: None,
            city: None,
            postcode: None,
            email: None,
        }
    }
}
