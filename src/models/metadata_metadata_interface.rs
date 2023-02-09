#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataMetadataInterface {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::MetadataMetadataInterfaceType>,
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<crate::models::MetadataMetadataInterfaceIp>,
    #[serde(rename = "mac", skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename = "vlan", skip_serializing_if = "Option::is_none")]
    pub vlan: Option<crate::models::MetadataMetadataVlan>,
}

impl MetadataMetadataInterface {
    pub fn new() -> MetadataMetadataInterface {
        MetadataMetadataInterface {
            _type: None,
            ipv4: None,
            mac: None,
            vlan: None,
        }
    }
}
