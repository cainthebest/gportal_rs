#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiPrice {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "formatted", skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
}

impl ApiPrice {
    pub fn new() -> ApiPrice {
        ApiPrice {
            amount: None,
            currency: None,
            formatted: None,
        }
    }
}
