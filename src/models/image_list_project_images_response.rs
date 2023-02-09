#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageListProjectImagesResponse {
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::ApiimageImage>>,
}

impl ImageListProjectImagesResponse {
    pub fn new() -> ImageListProjectImagesResponse {
        ImageListProjectImagesResponse { images: None }
    }
}
