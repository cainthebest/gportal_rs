#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProjectMember {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::ProjectProjectMemberUser>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::ProjectMemberRole>,
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "addedAt", skip_serializing_if = "Option::is_none")]
    pub added_at: Option<String>,
}

impl ProjectProjectMember {
    pub fn new() -> ProjectProjectMember {
        ProjectProjectMember {
            user: None,
            role: None,
            confirmed: None,
            default: None,
            added_at: None,
        }
    }
}
