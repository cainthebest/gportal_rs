#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserFinishWebAuthnRegistrationRequest {
    #[serde(
        rename = "credentialCreationResponse",
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_creation_response: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UserFinishWebAuthnRegistrationRequest {
    pub fn new() -> UserFinishWebAuthnRegistrationRequest {
        UserFinishWebAuthnRegistrationRequest {
            credential_creation_response: None,
            name: None,
        }
    }
}
