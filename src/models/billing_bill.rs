#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingBill {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "final", skip_serializing_if = "Option::is_none")]
    pub _final: Option<bool>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "vat", skip_serializing_if = "Option::is_none")]
    pub vat: Option<i32>,
    #[serde(rename = "net", skip_serializing_if = "Option::is_none")]
    pub net: Option<crate::models::ApiPrice>,
    #[serde(rename = "gross", skip_serializing_if = "Option::is_none")]
    pub gross: Option<crate::models::ApiPrice>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ApibillingBillStatus>,
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::models::PaymentPaymentMethod>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BillingBillItem>>,
    #[serde(rename = "outstandingNet", skip_serializing_if = "Option::is_none")]
    pub outstanding_net: Option<crate::models::ApiPrice>,
    #[serde(rename = "outstandingGross", skip_serializing_if = "Option::is_none")]
    pub outstanding_gross: Option<crate::models::ApiPrice>,
    #[serde(rename = "dueAt", skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::BasicBasicUser>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::BasicBasicProject>,
    #[serde(rename = "outstandingBalance", skip_serializing_if = "Option::is_none")]
    pub outstanding_balance: Option<crate::models::ApiPrice>,
}

impl BillingBill {
    pub fn new() -> BillingBill {
        BillingBill {
            id: None,
            number: None,
            _final: None,
            currency: None,
            vat: None,
            net: None,
            gross: None,
            status: None,
            payment_method: None,
            items: None,
            outstanding_net: None,
            outstanding_gross: None,
            due_at: None,
            created_at: None,
            user: None,
            project: None,
            outstanding_balance: None,
        }
    }
}
