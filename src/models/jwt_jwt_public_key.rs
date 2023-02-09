#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtJwtPublicKey {
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub _use: Option<String>,
    #[serde(rename = "x5t", skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
}

impl JwtJwtPublicKey {
    pub fn new() -> JwtJwtPublicKey {
        JwtJwtPublicKey {
            kid: None,
            kty: None,
            alg: None,
            _use: None,
            x5t: None,
            e: None,
            n: None,
        }
    }
}
