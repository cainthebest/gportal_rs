#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDeleteProjectImageVersionRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "imageVersionId", skip_serializing_if = "Option::is_none")]
    pub image_version_id: Option<String>,
}

impl ImageDeleteProjectImageVersionRequest {
    pub fn new() -> ImageDeleteProjectImageVersionRequest {
        ImageDeleteProjectImageVersionRequest {
            project_id: None,
            image_version_id: None,
        }
    }
}
