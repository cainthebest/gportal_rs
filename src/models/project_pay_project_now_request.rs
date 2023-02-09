#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectPayProjectNowRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl ProjectPayProjectNowRequest {
    pub fn new() -> ProjectPayProjectNowRequest {
        ProjectPayProjectNowRequest { project_id: None }
    }
}
