#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProjectSshKeyUser {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl ProjectProjectSshKeyUser {
    pub fn new() -> ProjectProjectSshKeyUser {
        ProjectProjectSshKeyUser {
            id: None,
            full_name: None,
            email: None,
            avatar: None,
        }
    }
}
