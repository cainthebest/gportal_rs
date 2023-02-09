#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectLeaveProjectRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl ProjectLeaveProjectRequest {
    pub fn new() -> ProjectLeaveProjectRequest {
        ProjectLeaveProjectRequest { project_id: None }
    }
}
