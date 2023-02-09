#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtJwtPublicCert {
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "cert", skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
}

impl JwtJwtPublicCert {
    pub fn new() -> JwtJwtPublicCert {
        JwtJwtPublicCert {
            kid: None,
            cert: None,
        }
    }
}
