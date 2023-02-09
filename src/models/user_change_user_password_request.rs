#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserChangeUserPasswordRequest {
    #[serde(rename = "currentPassword", skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    #[serde(rename = "newPassword", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
}

impl UserChangeUserPasswordRequest {
    pub fn new() -> UserChangeUserPasswordRequest {
        UserChangeUserPasswordRequest {
            current_password: None,
            new_password: None,
        }
    }
}
