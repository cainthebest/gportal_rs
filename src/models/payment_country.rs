#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentCountry {
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "netSupport", skip_serializing_if = "Option::is_none")]
    pub net_support: Option<bool>,
    #[serde(rename = "taxRate", skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<i32>,
}

impl PaymentCountry {
    pub fn new() -> PaymentCountry {
        PaymentCountry {
            country_code: None,
            name: None,
            net_support: None,
            tax_rate: None,
        }
    }
}
