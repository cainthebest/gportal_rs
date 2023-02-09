#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectProject {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<crate::models::ProjectProjectEnvironment>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::ProjectProjectMember>>,
    #[serde(rename = "creditCard", skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<crate::models::PaymentCreditCard>,
    #[serde(rename = "billingAddress", skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<crate::models::PaymentBillingAddress>,
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::models::PaymentPaymentMethod>,
    #[serde(
        rename = "availablePaymentMethods",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_payment_methods: Option<Vec<crate::models::PaymentPaymentMethod>>,
    #[serde(rename = "payment", skip_serializing_if = "Option::is_none")]
    pub payment: Option<bool>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ProjectProject {
    pub fn new() -> ProjectProject {
        ProjectProject {
            id: None,
            name: None,
            description: None,
            avatar: None,
            currency: None,
            environment: None,
            members: None,
            credit_card: None,
            billing_address: None,
            payment_method: None,
            available_payment_methods: None,
            payment: None,
            created_at: None,
            updated_at: None,
        }
    }
}
