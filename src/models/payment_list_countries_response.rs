#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentListCountriesResponse {
    #[serde(rename = "countries", skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<crate::models::PaymentCountry>>,
}

impl PaymentListCountriesResponse {
    pub fn new() -> PaymentListCountriesResponse {
        PaymentListCountriesResponse { countries: None }
    }
}
