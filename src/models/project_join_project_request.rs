#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectJoinProjectRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "accept", skip_serializing_if = "Option::is_none")]
    pub accept: Option<bool>,
}

impl ProjectJoinProjectRequest {
    pub fn new() -> ProjectJoinProjectRequest {
        ProjectJoinProjectRequest {
            project_id: None,
            accept: None,
        }
    }
}
