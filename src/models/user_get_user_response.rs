#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGetUserResponse {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::ApiuserUser>,
}

impl UserGetUserResponse {
    pub fn new() -> UserGetUserResponse {
        UserGetUserResponse { user: None }
    }
}
