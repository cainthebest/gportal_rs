#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserUpdateUserResponse {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::ApiuserUser>,
}

impl UserUpdateUserResponse {
    pub fn new() -> UserUpdateUserResponse {
        UserUpdateUserResponse { user: None }
    }
}
