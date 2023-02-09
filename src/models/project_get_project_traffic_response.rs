#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectGetProjectTrafficResponse {
    #[serde(rename = "bytesAvailable", skip_serializing_if = "Option::is_none")]
    pub bytes_available: Option<String>,
    #[serde(rename = "usages", skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<crate::models::ProjectProjectTrafficUsage>>,
    #[serde(
        rename = "additionalTrafficPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_traffic_price: Option<crate::models::ApiPrice>,
}

impl ProjectGetProjectTrafficResponse {
    pub fn new() -> ProjectGetProjectTrafficResponse {
        ProjectGetProjectTrafficResponse {
            bytes_available: None,
            usages: None,
            additional_traffic_price: None,
        }
    }
}
