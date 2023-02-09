#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLongLivedToken {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl UserLongLivedToken {
    pub fn new() -> UserLongLivedToken {
        UserLongLivedToken {
            id: None,
            token_id: None,
            access_token: None,
            description: None,
            created_at: None,
            expires_at: None,
        }
    }
}
