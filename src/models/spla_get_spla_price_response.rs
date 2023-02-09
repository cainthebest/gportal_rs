#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplaGetSplaPriceResponse {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::models::SplaSplaPrice>,
}

impl SplaGetSplaPriceResponse {
    pub fn new() -> SplaGetSplaPriceResponse {
        SplaGetSplaPriceResponse { price: None }
    }
}
