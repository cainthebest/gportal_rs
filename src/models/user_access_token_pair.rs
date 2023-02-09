#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccessTokenPair {
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "refreshToken", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl UserAccessTokenPair {
    pub fn new() -> UserAccessTokenPair {
        UserAccessTokenPair {
            access_token: None,
            refresh_token: None,
        }
    }
}
