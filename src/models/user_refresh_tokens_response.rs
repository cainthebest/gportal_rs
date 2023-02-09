#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRefreshTokensResponse {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::ApiuserUser>,
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl UserRefreshTokensResponse {
    pub fn new() -> UserRefreshTokensResponse {
        UserRefreshTokensResponse {
            user: None,
            access_token: None,
            refresh_token: None,
        }
    }
}
