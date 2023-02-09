#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSwitch {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "netboxLink", skip_serializing_if = "Option::is_none")]
    pub netbox_link: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::NetworkSwitchType>,
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<crate::models::RegionDataCenter>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "serverInterfaces", skip_serializing_if = "Option::is_none")]
    pub server_interfaces: Option<Vec<crate::models::NetworkServerInterface>>,
    #[serde(rename = "uplinkPorts", skip_serializing_if = "Option::is_none")]
    pub uplink_ports: Option<Vec<String>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "lastArpSync", skip_serializing_if = "Option::is_none")]
    pub last_arp_sync: Option<String>,
}

impl NetworkSwitch {
    pub fn new() -> NetworkSwitch {
        NetworkSwitch {
            id: None,
            name: None,
            netbox_link: None,
            _type: None,
            datacenter: None,
            ip: None,
            username: None,
            password: None,
            server_interfaces: None,
            uplink_ports: None,
            created_at: None,
            updated_at: None,
            last_arp_sync: None,
        }
    }
}
