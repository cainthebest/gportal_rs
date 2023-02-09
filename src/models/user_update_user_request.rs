#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateUserRequest {
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

impl UserUpdateUserRequest {
    pub fn new() -> UserUpdateUserRequest {
        UserUpdateUserRequest { full_name: None }
    }
}
