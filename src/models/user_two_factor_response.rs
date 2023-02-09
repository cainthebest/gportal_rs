#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTwoFactorResponse {
    #[serde(rename = "twoFactorMethods", skip_serializing_if = "Option::is_none")]
    pub two_factor_methods: Option<Vec<crate::models::UserTwoFactorMethod>>,
    #[serde(rename = "credentialCreation", skip_serializing_if = "Option::is_none")]
    pub credential_creation: Option<String>,
}

impl UserTwoFactorResponse {
    pub fn new() -> UserTwoFactorResponse {
        UserTwoFactorResponse {
            two_factor_methods: None,
            credential_creation: None,
        }
    }
}
