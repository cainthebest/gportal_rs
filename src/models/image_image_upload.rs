#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageImageUpload {
    #[serde(rename = "uploadUrl", skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl ImageImageUpload {
    pub fn new() -> ImageImageUpload {
        ImageImageUpload {
            upload_url: None,
            token: None,
        }
    }
}
