#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataMetadataPasswordResponse {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl MetadataMetadataPasswordResponse {
    pub fn new() -> MetadataMetadataPasswordResponse {
        MetadataMetadataPasswordResponse { password: None }
    }
}
