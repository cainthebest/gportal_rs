#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerGetVncResponse {
    #[serde(rename = "websocket", skip_serializing_if = "Option::is_none")]
    pub websocket: Option<String>,
}

impl ServerGetVncResponse {
    pub fn new() -> ServerGetVncResponse {
        ServerGetVncResponse { websocket: None }
    }
}
