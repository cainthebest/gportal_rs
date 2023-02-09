#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRevokeLongLivedTokenRequest {
    #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
}

impl UserRevokeLongLivedTokenRequest {
    pub fn new() -> UserRevokeLongLivedTokenRequest {
        UserRevokeLongLivedTokenRequest { token_id: None }
    }
}
