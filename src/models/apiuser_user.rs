#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiuserUser {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "computeLimit", skip_serializing_if = "Option::is_none")]
    pub compute_limit: Option<i32>,
    #[serde(rename = "paymentMethods", skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<crate::models::PaymentPaymentMethod>>,
    #[serde(rename = "lastLogin", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ApiuserUser {
    pub fn new() -> ApiuserUser {
        ApiuserUser {
            id: None,
            number: None,
            full_name: None,
            email: None,
            confirmed: None,
            locked: None,
            admin: None,
            avatar: None,
            compute_limit: None,
            payment_methods: None,
            last_login: None,
            created_at: None,
            updated_at: None,
        }
    }
}
