#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkIpAddress {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::models::NetworkCidr>,
    #[serde(rename = "subnet", skip_serializing_if = "Option::is_none")]
    pub subnet: Option<crate::models::NetworkSubnet>,
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "online", skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}

impl NetworkIpAddress {
    pub fn new() -> NetworkIpAddress {
        NetworkIpAddress {
            address: None,
            subnet: None,
            primary: None,
            online: None,
        }
    }
}
