#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectGetProjectsOutstandingBalanceResponse {
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<crate::models::ProjectOutstandingBalance>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::models::ApiPrice>,
}

impl ProjectGetProjectsOutstandingBalanceResponse {
    pub fn new() -> ProjectGetProjectsOutstandingBalanceResponse {
        ProjectGetProjectsOutstandingBalanceResponse {
            balances: None,
            total: None,
        }
    }
}
