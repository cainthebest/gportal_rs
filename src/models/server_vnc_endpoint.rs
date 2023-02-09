#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerVncEndpoint {
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl ServerVncEndpoint {
    pub fn new() -> ServerVncEndpoint {
        ServerVncEndpoint {
            ip_address: None,
            port: None,
            password: None,
        }
    }
}
