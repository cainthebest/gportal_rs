#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserListSessionsResponse {
    #[serde(rename = "sessions", skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<crate::models::UserUserSession>>,
}

impl UserListSessionsResponse {
    pub fn new() -> UserListSessionsResponse {
        UserListSessionsResponse { sessions: None }
    }
}
