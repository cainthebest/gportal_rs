#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlavourGetProjectFlavoursResponse {
    #[serde(rename = "flavours", skip_serializing_if = "Option::is_none")]
    pub flavours: Option<Vec<crate::models::FlavourFlavour>>,
}

impl FlavourGetProjectFlavoursResponse {
    pub fn new() -> FlavourGetProjectFlavoursResponse {
        FlavourGetProjectFlavoursResponse { flavours: None }
    }
}
