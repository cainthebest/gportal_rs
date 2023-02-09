#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectGetProjectBillsResponse {
    #[serde(rename = "bills", skip_serializing_if = "Option::is_none")]
    pub bills: Option<Vec<crate::models::BillingBill>>,
    #[serde(rename = "years", skip_serializing_if = "Option::is_none")]
    pub years: Option<Vec<i32>>,
}

impl ProjectGetProjectBillsResponse {
    pub fn new() -> ProjectGetProjectBillsResponse {
        ProjectGetProjectBillsResponse {
            bills: None,
            years: None,
        }
    }
}
