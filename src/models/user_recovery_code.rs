#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRecoveryCode {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "usedAt", skip_serializing_if = "Option::is_none")]
    pub used_at: Option<String>,
}

impl UserRecoveryCode {
    pub fn new() -> UserRecoveryCode {
        UserRecoveryCode {
            id: None,
            code: None,
            used_at: None,
        }
    }
}
