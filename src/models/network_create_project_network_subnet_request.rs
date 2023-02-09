#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkCreateProjectNetworkSubnetRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "cidr", skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(rename = "poolStart", skip_serializing_if = "Option::is_none")]
    pub pool_start: Option<String>,
    #[serde(rename = "poolEnd", skip_serializing_if = "Option::is_none")]
    pub pool_end: Option<String>,
}

impl NetworkCreateProjectNetworkSubnetRequest {
    pub fn new() -> NetworkCreateProjectNetworkSubnetRequest {
        NetworkCreateProjectNetworkSubnetRequest {
            project_id: None,
            id: None,
            cidr: None,
            pool_start: None,
            pool_end: None,
        }
    }
}
