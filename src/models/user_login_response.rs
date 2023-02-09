#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginResponse {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::ApiuserUser>,
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<crate::models::UserAccessTokenPair>,
    #[serde(rename = "twoFactor", skip_serializing_if = "Option::is_none")]
    pub two_factor: Option<crate::models::UserTwoFactorResponse>,
}

impl UserLoginResponse {
    pub fn new() -> UserLoginResponse {
        UserLoginResponse {
            user: None,
            tokens: None,
            two_factor: None,
        }
    }
}
