#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplaSplaPrice {
    #[serde(rename = "basePrice", skip_serializing_if = "Option::is_none")]
    pub base_price: Option<crate::models::ApiPrice>,
    #[serde(
        rename = "additional2CoresPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional2_cores_price: Option<crate::models::ApiPrice>,
}

impl SplaSplaPrice {
    pub fn new() -> SplaSplaPrice {
        SplaSplaPrice {
            base_price: None,
            additional2_cores_price: None,
        }
    }
}
