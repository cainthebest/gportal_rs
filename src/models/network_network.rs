#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkNetwork {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::NetworkNetworkType>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::BasicBasicProject>,
    #[serde(rename = "subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<crate::models::NetworkSubnet>>,
    #[serde(rename = "vlanId", skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<i32>,
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<crate::models::RegionDataCenter>,
    #[serde(rename = "poolSize", skip_serializing_if = "Option::is_none")]
    pub pool_size: Option<String>,
    #[serde(rename = "poolAvailable", skip_serializing_if = "Option::is_none")]
    pub pool_available: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl NetworkNetwork {
    pub fn new() -> NetworkNetwork {
        NetworkNetwork {
            id: None,
            name: None,
            _type: None,
            project: None,
            subnets: None,
            vlan_id: None,
            datacenter: None,
            pool_size: None,
            pool_available: None,
            created_at: None,
            updated_at: None,
        }
    }
}
