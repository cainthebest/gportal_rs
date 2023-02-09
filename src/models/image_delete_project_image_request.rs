#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDeleteProjectImageRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

impl ImageDeleteProjectImageRequest {
    pub fn new() -> ImageDeleteProjectImageRequest {
        ImageDeleteProjectImageRequest {
            project_id: None,
            image_id: None,
        }
    }
}
