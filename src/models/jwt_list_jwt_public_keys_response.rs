#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtListJwtPublicKeysResponse {
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<crate::models::JwtJwtPublicKey>>,
    #[serde(rename = "publicCert", skip_serializing_if = "Option::is_none")]
    pub public_cert: Option<crate::models::JwtJwtPublicCert>,
    #[serde(rename = "publicCerts", skip_serializing_if = "Option::is_none")]
    pub public_certs: Option<Vec<crate::models::JwtJwtPublicCert>>,
}

impl JwtListJwtPublicKeysResponse {
    pub fn new() -> JwtListJwtPublicKeysResponse {
        JwtListJwtPublicKeysResponse {
            keys: None,
            public_cert: None,
            public_certs: None,
        }
    }
}
