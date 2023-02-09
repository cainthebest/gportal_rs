#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerListVncEndpointsResponse {
    #[serde(rename = "endpoints", skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<crate::models::ServerVncEndpoint>>,
}

impl ServerListVncEndpointsResponse {
    pub fn new() -> ServerListVncEndpointsResponse {
        ServerListVncEndpointsResponse { endpoints: None }
    }
}
