#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkListProjectNetworksResponse {
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkNetwork>>,
}

impl NetworkListProjectNetworksResponse {
    pub fn new() -> NetworkListProjectNetworksResponse {
        NetworkListProjectNetworksResponse { networks: None }
    }
}
