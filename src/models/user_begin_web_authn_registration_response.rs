#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserBeginWebAuthnRegistrationResponse {
    #[serde(rename = "credentialCreation", skip_serializing_if = "Option::is_none")]
    pub credential_creation: Option<String>,
}

impl UserBeginWebAuthnRegistrationResponse {
    pub fn new() -> UserBeginWebAuthnRegistrationResponse {
        UserBeginWebAuthnRegistrationResponse {
            credential_creation: None,
        }
    }
}
