#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageImageVersion {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ImageImageType>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(rename = "compressed", skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ImageImageVersion {
    pub fn new() -> ImageImageVersion {
        ImageImageVersion {
            id: None,
            _type: None,
            size: None,
            available: None,
            compressed: None,
            checksum: None,
            created_at: None,
            updated_at: None,
        }
    }
}
