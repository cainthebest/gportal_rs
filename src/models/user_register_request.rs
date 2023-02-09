#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRegisterRequest {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "recaptchaToken", skip_serializing_if = "Option::is_none")]
    pub recaptcha_token: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<crate::models::ApiPlatform>,
}

impl UserRegisterRequest {
    pub fn new() -> UserRegisterRequest {
        UserRegisterRequest {
            email: None,
            password: None,
            recaptcha_token: None,
            platform: None,
        }
    }
}
