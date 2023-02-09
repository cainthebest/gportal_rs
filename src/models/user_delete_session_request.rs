#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDeleteSessionRequest {
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl UserDeleteSessionRequest {
    pub fn new() -> UserDeleteSessionRequest {
        UserDeleteSessionRequest { session_id: None }
    }
}
