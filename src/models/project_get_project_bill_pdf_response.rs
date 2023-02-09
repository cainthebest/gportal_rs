#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectGetProjectBillPdfResponse {
    #[serde(rename = "downloadUrl", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

impl ProjectGetProjectBillPdfResponse {
    pub fn new() -> ProjectGetProjectBillPdfResponse {
        ProjectGetProjectBillPdfResponse { download_url: None }
    }
}
