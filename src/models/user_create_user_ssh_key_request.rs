#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateUserSshKeyRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

impl UserCreateUserSshKeyRequest {
    pub fn new() -> UserCreateUserSshKeyRequest {
        UserCreateUserSshKeyRequest {
            name: None,
            public_key: None,
        }
    }
}
