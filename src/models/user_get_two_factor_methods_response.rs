#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGetTwoFactorMethodsResponse {
    #[serde(rename = "enabledMethods", skip_serializing_if = "Option::is_none")]
    pub enabled_methods: Option<Vec<crate::models::UserTwoFactorMethod>>,
    #[serde(rename = "webauthnDevices", skip_serializing_if = "Option::is_none")]
    pub webauthn_devices: Option<Vec<crate::models::UserWebAuthn>>,
}

impl UserGetTwoFactorMethodsResponse {
    pub fn new() -> UserGetTwoFactorMethodsResponse {
        UserGetTwoFactorMethodsResponse {
            enabled_methods: None,
            webauthn_devices: None,
        }
    }
}
