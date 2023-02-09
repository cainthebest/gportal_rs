/// NetworkCidr : Cidr This contains the IP version and IP inclusive prefix.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkCidr {
    #[serde(rename = "ipVersion", skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<crate::models::CidrVersion>,
    #[serde(rename = "cidr", skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

impl NetworkCidr {
    /// Cidr This contains the IP version and IP inclusive prefix.
    pub fn new() -> NetworkCidr {
        NetworkCidr {
            ip_version: None,
            cidr: None,
        }
    }
}
