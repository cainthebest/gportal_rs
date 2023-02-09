#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectChangeDefaultProjectRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl ProjectChangeDefaultProjectRequest {
    pub fn new() -> ProjectChangeDefaultProjectRequest {
        ProjectChangeDefaultProjectRequest { project_id: None }
    }
}
