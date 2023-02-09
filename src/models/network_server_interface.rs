#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkServerInterface {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::NetworkServerInterfaceType>,
    #[serde(rename = "macAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<i32>,
    #[serde(rename = "ips", skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<crate::models::NetworkIpAddress>>,
    #[serde(rename = "serverId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "switch", skip_serializing_if = "Option::is_none")]
    pub switch: Option<crate::models::NetworkSwitch>,
    #[serde(rename = "switchPort", skip_serializing_if = "Option::is_none")]
    pub switch_port: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl NetworkServerInterface {
    pub fn new() -> NetworkServerInterface {
        NetworkServerInterface {
            id: None,
            _type: None,
            mac_address: None,
            driver: None,
            speed: None,
            ips: None,
            server_id: None,
            switch: None,
            switch_port: None,
            created_at: None,
            updated_at: None,
        }
    }
}
