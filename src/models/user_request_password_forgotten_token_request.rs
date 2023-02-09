#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRequestPasswordForgottenTokenRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl UserRequestPasswordForgottenTokenRequest {
    pub fn new() -> UserRequestPasswordForgottenTokenRequest {
        UserRequestPasswordForgottenTokenRequest { email: None }
    }
}
