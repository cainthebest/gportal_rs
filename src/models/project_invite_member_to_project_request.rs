#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectInviteMemberToProjectRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl ProjectInviteMemberToProjectRequest {
    pub fn new() -> ProjectInviteMemberToProjectRequest {
        ProjectInviteMemberToProjectRequest {
            project_id: None,
            email: None,
        }
    }
}
