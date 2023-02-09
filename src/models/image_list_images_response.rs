#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageListImagesResponse {
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::ApiimageImage>>,
}

impl ImageListImagesResponse {
    pub fn new() -> ImageListImagesResponse {
        ImageListImagesResponse { images: None }
    }
}
