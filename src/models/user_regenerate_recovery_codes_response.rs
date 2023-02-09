#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRegenerateRecoveryCodesResponse {
    #[serde(rename = "recoveryCodes", skip_serializing_if = "Option::is_none")]
    pub recovery_codes: Option<Vec<crate::models::UserRecoveryCode>>,
}

impl UserRegenerateRecoveryCodesResponse {
    pub fn new() -> UserRegenerateRecoveryCodesResponse {
        UserRegenerateRecoveryCodesResponse {
            recovery_codes: None,
        }
    }
}
