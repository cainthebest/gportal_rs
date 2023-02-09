#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConfirmEMailRequest {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl UserConfirmEMailRequest {
    pub fn new() -> UserConfirmEMailRequest {
        UserConfirmEMailRequest { token: None }
    }
}
