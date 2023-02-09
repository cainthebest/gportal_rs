#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkUpdateProjectNetworkRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl NetworkUpdateProjectNetworkRequest {
    pub fn new() -> NetworkUpdateProjectNetworkRequest {
        NetworkUpdateProjectNetworkRequest {
            project_id: None,
            id: None,
            name: None,
        }
    }
}
