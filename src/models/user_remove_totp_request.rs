#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRemoveTotpRequest {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UserRemoveTotpRequest {
    pub fn new() -> UserRemoveTotpRequest {
        UserRemoveTotpRequest { password: None }
    }
}
