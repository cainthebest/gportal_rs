#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResetUserPasswordRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UserResetUserPasswordRequest {
    pub fn new() -> UserResetUserPasswordRequest {
        UserResetUserPasswordRequest {
            email: None,
            token: None,
            password: None,
        }
    }
}
