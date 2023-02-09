#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataMetadataDns {
    #[serde(rename = "nameservers", skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,
}

impl MetadataMetadataDns {
    pub fn new() -> MetadataMetadataDns {
        MetadataMetadataDns {
            nameservers: None,
            search: None,
        }
    }
}
