#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkMacAddressMapping {
    #[serde(rename = "macAddresses", skip_serializing_if = "Option::is_none")]
    pub mac_addresses: Option<Vec<String>>,
    #[serde(rename = "switchId", skip_serializing_if = "Option::is_none")]
    pub switch_id: Option<String>,
    #[serde(rename = "switchPort", skip_serializing_if = "Option::is_none")]
    pub switch_port: Option<String>,
}

impl NetworkMacAddressMapping {
    pub fn new() -> NetworkMacAddressMapping {
        NetworkMacAddressMapping {
            mac_addresses: None,
            switch_id: None,
            switch_port: None,
        }
    }
}
