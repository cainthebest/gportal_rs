#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeGetComputeResourcePricingResponse {
    #[serde(rename = "currentPrice", skip_serializing_if = "Option::is_none")]
    pub current_price: Option<crate::models::ApiPrice>,
    #[serde(
        rename = "currentBillingPeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_billing_period: Option<crate::models::BillingBillingPeriod>,
    #[serde(rename = "newMonthlyPricing", skip_serializing_if = "Option::is_none")]
    pub new_monthly_pricing: Option<crate::models::ApiPrice>,
    #[serde(rename = "newHourlyPricing", skip_serializing_if = "Option::is_none")]
    pub new_hourly_pricing: Option<crate::models::ApiPrice>,
}

impl ComputeGetComputeResourcePricingResponse {
    pub fn new() -> ComputeGetComputeResourcePricingResponse {
        ComputeGetComputeResourcePricingResponse {
            current_price: None,
            current_billing_period: None,
            new_monthly_pricing: None,
            new_hourly_pricing: None,
        }
    }
}
