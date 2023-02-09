#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectListProjectsResponse {
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<crate::models::ProjectProject>>,
    #[serde(rename = "invites", skip_serializing_if = "Option::is_none")]
    pub invites: Option<Vec<crate::models::ProjectProjectInvite>>,
}

impl ProjectListProjectsResponse {
    pub fn new() -> ProjectListProjectsResponse {
        ProjectListProjectsResponse {
            projects: None,
            invites: None,
        }
    }
}
