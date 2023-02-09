#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerServer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<crate::models::RegionDataCenter>,
    #[serde(rename = "netboxLink", skip_serializing_if = "Option::is_none")]
    pub netbox_link: Option<String>,
    #[serde(rename = "flavour", skip_serializing_if = "Option::is_none")]
    pub flavour: Option<crate::models::FlavourFlavour>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ServerServerStatus>,
    #[serde(rename = "statusUpdatedAt", skip_serializing_if = "Option::is_none")]
    pub status_updated_at: Option<String>,
    #[serde(rename = "interfaces", skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<crate::models::NetworkServerInterface>>,
    #[serde(
        rename = "platformManagementType",
        skip_serializing_if = "Option::is_none"
    )]
    pub platform_management_type: Option<crate::models::ServerPlatformManagementType>,
    #[serde(rename = "serverAlert", skip_serializing_if = "Option::is_none")]
    pub server_alert: Option<bool>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "inPool", skip_serializing_if = "Option::is_none")]
    pub in_pool: Option<bool>,
    #[serde(rename = "lastSyncAt", skip_serializing_if = "Option::is_none")]
    pub last_sync_at: Option<String>,
    #[serde(rename = "hasVncConsole", skip_serializing_if = "Option::is_none")]
    pub has_vnc_console: Option<bool>,
}

impl ServerServer {
    pub fn new() -> ServerServer {
        ServerServer {
            id: None,
            name: None,
            datacenter: None,
            netbox_link: None,
            flavour: None,
            status: None,
            status_updated_at: None,
            interfaces: None,
            platform_management_type: None,
            server_alert: None,
            created_at: None,
            updated_at: None,
            in_pool: None,
            last_sync_at: None,
            has_vnc_console: None,
        }
    }
}
