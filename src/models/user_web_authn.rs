#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserWebAuthn {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "authenticator", skip_serializing_if = "Option::is_none")]
    pub authenticator: Option<crate::models::UserWebAuthnAuthenticator>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastUsedAt", skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
}

impl UserWebAuthn {
    pub fn new() -> UserWebAuthn {
        UserWebAuthn {
            id: None,
            name: None,
            authenticator: None,
            created_at: None,
            last_used_at: None,
        }
    }
}
