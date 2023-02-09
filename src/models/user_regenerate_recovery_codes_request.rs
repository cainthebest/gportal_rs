#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRegenerateRecoveryCodesRequest {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UserRegenerateRecoveryCodesRequest {
    pub fn new() -> UserRegenerateRecoveryCodesRequest {
        UserRegenerateRecoveryCodesRequest { password: None }
    }
}
