#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDeleteWebAuthnDeviceRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UserDeleteWebAuthnDeviceRequest {
    pub fn new() -> UserDeleteWebAuthnDeviceRequest {
        UserDeleteWebAuthnDeviceRequest {
            id: None,
            password: None,
        }
    }
}
