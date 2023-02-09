#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataMetadata {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ApiComputeResourceType>,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "vendorData", skip_serializing_if = "Option::is_none")]
    pub vendor_data: Option<String>,
    #[serde(rename = "userData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "publicKeys", skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<String>>,
    #[serde(rename = "interfaces", skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<crate::models::MetadataMetadataInterface>>,
    #[serde(rename = "dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<crate::models::MetadataMetadataDns>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl MetadataMetadata {
    pub fn new() -> MetadataMetadata {
        MetadataMetadata {
            id: None,
            _type: None,
            hostname: None,
            vendor_data: None,
            user_data: None,
            region: None,
            public_keys: None,
            interfaces: None,
            dns: None,
            tags: None,
        }
    }
}
