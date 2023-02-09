#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSubnet {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::models::NetworkNetwork>,
    #[serde(rename = "cidr", skip_serializing_if = "Option::is_none")]
    pub cidr: Option<crate::models::NetworkCidr>,
    #[serde(rename = "gatewayIp", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<crate::models::NetworkCidr>,
    #[serde(rename = "dhcp", skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,
    #[serde(rename = "poolStart", skip_serializing_if = "Option::is_none")]
    pub pool_start: Option<crate::models::NetworkCidr>,
    #[serde(rename = "poolEnd", skip_serializing_if = "Option::is_none")]
    pub pool_end: Option<crate::models::NetworkCidr>,
    #[serde(rename = "poolSize", skip_serializing_if = "Option::is_none")]
    pub pool_size: Option<String>,
    #[serde(rename = "poolAvailable", skip_serializing_if = "Option::is_none")]
    pub pool_available: Option<String>,
}

impl NetworkSubnet {
    pub fn new() -> NetworkSubnet {
        NetworkSubnet {
            id: None,
            network: None,
            cidr: None,
            gateway_ip: None,
            dhcp: None,
            pool_start: None,
            pool_end: None,
            pool_size: None,
            pool_available: None,
        }
    }
}
