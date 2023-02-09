#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRefreshTokensRequest {
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl UserRefreshTokensRequest {
    pub fn new() -> UserRefreshTokensRequest {
        UserRefreshTokensRequest {
            refresh_token: None,
        }
    }
}
