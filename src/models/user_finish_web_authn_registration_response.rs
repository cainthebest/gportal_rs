#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserFinishWebAuthnRegistrationResponse {
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<crate::models::UserWebAuthn>,
}

impl UserFinishWebAuthnRegistrationResponse {
    pub fn new() -> UserFinishWebAuthnRegistrationResponse {
        UserFinishWebAuthnRegistrationResponse { device: None }
    }
}
