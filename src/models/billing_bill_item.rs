#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingBillItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<crate::models::BillingProduct>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::models::ApiPrice>,
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "endedAt", skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(rename = "billingPeriod", skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<crate::models::BillingBillingPeriod>,
}

impl BillingBillItem {
    pub fn new() -> BillingBillItem {
        BillingBillItem {
            id: None,
            product: None,
            description: None,
            price: None,
            started_at: None,
            ended_at: None,
            billing_period: None,
        }
    }
}
