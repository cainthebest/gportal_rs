#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateProjectRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<crate::models::ProjectProjectEnvironment>,
    #[serde(rename = "creditCardId", skip_serializing_if = "Option::is_none")]
    pub credit_card_id: Option<String>,
    #[serde(rename = "billingAddressId", skip_serializing_if = "Option::is_none")]
    pub billing_address_id: Option<String>,
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::models::PaymentPaymentMethod>,
}

impl ProjectUpdateProjectRequest {
    pub fn new() -> ProjectUpdateProjectRequest {
        ProjectUpdateProjectRequest {
            project_id: None,
            name: None,
            description: None,
            environment: None,
            credit_card_id: None,
            billing_address_id: None,
            payment_method: None,
        }
    }
}
