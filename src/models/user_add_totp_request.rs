#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAddTotpRequest {
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UserAddTotpRequest {
    pub fn new() -> UserAddTotpRequest {
        UserAddTotpRequest {
            secret: None,
            code: None,
            password: None,
        }
    }
}
