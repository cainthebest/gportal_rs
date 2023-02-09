#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGetUserComputeLimitResponse {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::models::UserUserComputeLimit>,
}

impl UserGetUserComputeLimitResponse {
    pub fn new() -> UserGetUserComputeLimitResponse {
        UserGetUserComputeLimitResponse { limit: None }
    }
}
