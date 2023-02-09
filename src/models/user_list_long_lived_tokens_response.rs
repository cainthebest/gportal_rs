#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserListLongLivedTokensResponse {
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<crate::models::UserLongLivedToken>>,
}

impl UserListLongLivedTokensResponse {
    pub fn new() -> UserListLongLivedTokensResponse {
        UserListLongLivedTokensResponse { tokens: None }
    }
}
