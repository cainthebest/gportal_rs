#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataMetadataInterfaceIp {
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "netmask", skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

impl MetadataMetadataInterfaceIp {
    pub fn new() -> MetadataMetadataInterfaceIp {
        MetadataMetadataInterfaceIp {
            ip_address: None,
            netmask: None,
            prefix: None,
            gateway: None,
        }
    }
}
