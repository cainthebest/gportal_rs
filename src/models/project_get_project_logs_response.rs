#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectGetProjectLogsResponse {
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<crate::models::ProjectProjectLog>>,
    #[serde(rename = "pagesTotal", skip_serializing_if = "Option::is_none")]
    pub pages_total: Option<i32>,
}

impl ProjectGetProjectLogsResponse {
    pub fn new() -> ProjectGetProjectLogsResponse {
        ProjectGetProjectLogsResponse {
            logs: None,
            pages_total: None,
        }
    }
}
