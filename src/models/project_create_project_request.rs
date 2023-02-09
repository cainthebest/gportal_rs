#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreateProjectRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<crate::models::ProjectProjectEnvironment>,
}

impl ProjectCreateProjectRequest {
    pub fn new() -> ProjectCreateProjectRequest {
        ProjectCreateProjectRequest {
            name: None,
            description: None,
            environment: None,
        }
    }
}
