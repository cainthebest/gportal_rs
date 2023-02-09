#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectListProjectSshKeysResponse {
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<crate::models::ProjectProjectSshKey>>,
}

impl ProjectListProjectSshKeysResponse {
    pub fn new() -> ProjectListProjectSshKeysResponse {
        ProjectListProjectSshKeysResponse { keys: None }
    }
}
