#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectOutstandingBalance {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<crate::models::ApiPrice>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::ProjectProject>,
}

impl ProjectOutstandingBalance {
    pub fn new() -> ProjectOutstandingBalance {
        ProjectOutstandingBalance {
            amount: None,
            project: None,
        }
    }
}
