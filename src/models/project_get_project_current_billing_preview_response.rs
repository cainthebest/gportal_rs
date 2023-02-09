#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectGetProjectCurrentBillingPreviewResponse {
    #[serde(rename = "bill", skip_serializing_if = "Option::is_none")]
    pub bill: Option<crate::models::BillingBill>,
}

impl ProjectGetProjectCurrentBillingPreviewResponse {
    pub fn new() -> ProjectGetProjectCurrentBillingPreviewResponse {
        ProjectGetProjectCurrentBillingPreviewResponse { bill: None }
    }
}
