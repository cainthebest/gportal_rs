#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiFile {
    #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl ApiFile {
    pub fn new() -> ApiFile {
        ApiFile {
            bytes: None,
            mime_type: None,
        }
    }
}
