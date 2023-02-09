#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateTotpResponse {
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "qrCode", skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<crate::models::ApiFile>,
}

impl UserCreateTotpResponse {
    pub fn new() -> UserCreateTotpResponse {
        UserCreateTotpResponse {
            secret: None,
            qr_code: None,
        }
    }
}
