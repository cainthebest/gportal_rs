#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProjectLog {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::ApiuserUser>,
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ProjectProjectLog {
    pub fn new() -> ProjectProjectLog {
        ProjectProjectLog {
            id: None,
            user: None,
            log: None,
            created_at: None,
            updated_at: None,
        }
    }
}
