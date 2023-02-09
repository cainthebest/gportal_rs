#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProjectSshKey {
    #[serde(rename = "sshKey", skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<crate::models::SecuritySshKey>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::ProjectProjectSshKeyUser>,
}

impl ProjectProjectSshKey {
    pub fn new() -> ProjectProjectSshKey {
        ProjectProjectSshKey {
            ssh_key: None,
            user: None,
        }
    }
}
