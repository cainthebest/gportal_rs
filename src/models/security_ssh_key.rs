#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySshKey {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "bits", skip_serializing_if = "Option::is_none")]
    pub bits: Option<i32>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl SecuritySshKey {
    pub fn new() -> SecuritySshKey {
        SecuritySshKey {
            id: None,
            name: None,
            public_key: None,
            fingerprint: None,
            _type: None,
            bits: None,
            created_at: None,
        }
    }
}
