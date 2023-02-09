#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserUserComputeLimit {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
}

impl UserUserComputeLimit {
    pub fn new() -> UserUserComputeLimit {
        UserUserComputeLimit {
            limit: None,
            current: None,
        }
    }
}
