#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectGetProjectCurrentBillingPreviewPdfResponse {
    #[serde(rename = "downloadUrl", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

impl ProjectGetProjectCurrentBillingPreviewPdfResponse {
    pub fn new() -> ProjectGetProjectCurrentBillingPreviewPdfResponse {
        ProjectGetProjectCurrentBillingPreviewPdfResponse { download_url: None }
    }
}
