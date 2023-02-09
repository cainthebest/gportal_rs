#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLoginRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "totp", skip_serializing_if = "Option::is_none")]
    pub totp: Option<String>,
    #[serde(rename = "recoveryCode", skip_serializing_if = "Option::is_none")]
    pub recovery_code: Option<String>,
    #[serde(
        rename = "credentialCreationResponse",
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_creation_response: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<crate::models::ApiPlatform>,
}

impl UserLoginRequest {
    pub fn new() -> UserLoginRequest {
        UserLoginRequest {
            email: None,
            password: None,
            totp: None,
            recovery_code: None,
            credential_creation_response: None,
            platform: None,
        }
    }
}
