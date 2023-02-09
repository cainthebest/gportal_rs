#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProjectInvite {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "addedAt", skip_serializing_if = "Option::is_none")]
    pub added_at: Option<String>,
}

impl ProjectProjectInvite {
    pub fn new() -> ProjectProjectInvite {
        ProjectProjectInvite {
            id: None,
            name: None,
            avatar: None,
            added_at: None,
        }
    }
}
