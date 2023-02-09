#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataMetadataVlan {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "tagged", skip_serializing_if = "Option::is_none")]
    pub tagged: Option<bool>,
}

impl MetadataMetadataVlan {
    pub fn new() -> MetadataMetadataVlan {
        MetadataMetadataVlan {
            id: None,
            tagged: None,
        }
    }
}
