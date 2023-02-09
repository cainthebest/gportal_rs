#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserUserSession {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "asn", skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<crate::models::ApiPlatform>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl UserUserSession {
    pub fn new() -> UserUserSession {
        UserUserSession {
            id: None,
            ip_address: None,
            asn: None,
            country_code: None,
            city: None,
            platform: None,
            created_at: None,
            expires_at: None,
        }
    }
}
