#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeChangeComputeResourceBillingPeriodRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "billingPeriod", skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<crate::models::BillingBillingPeriod>,
}

impl ComputeChangeComputeResourceBillingPeriodRequest {
    pub fn new() -> ComputeChangeComputeResourceBillingPeriodRequest {
        ComputeChangeComputeResourceBillingPeriodRequest {
            project_id: None,
            resource_id: None,
            billing_period: None,
        }
    }
}
