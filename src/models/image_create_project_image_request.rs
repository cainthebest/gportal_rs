#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageCreateProjectImageRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "authenticationTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_types: Option<Vec<crate::models::ImageAuthenticationType>>,
}

impl ImageCreateProjectImageRequest {
    pub fn new() -> ImageCreateProjectImageRequest {
        ImageCreateProjectImageRequest {
            project_id: None,
            name: None,
            authentication_types: None,
        }
    }
}
