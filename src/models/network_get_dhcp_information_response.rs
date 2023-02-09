#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkGetDhcpInformationResponse {
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "dnsServers", skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    #[serde(rename = "ntpServers", skip_serializing_if = "Option::is_none")]
    pub ntp_servers: Option<Vec<String>>,
    #[serde(rename = "leaseTtl", skip_serializing_if = "Option::is_none")]
    pub lease_ttl: Option<String>,
    #[serde(rename = "bootTftp", skip_serializing_if = "Option::is_none")]
    pub boot_tftp: Option<bool>,
}

impl NetworkGetDhcpInformationResponse {
    pub fn new() -> NetworkGetDhcpInformationResponse {
        NetworkGetDhcpInformationResponse {
            ip_address: None,
            gateway: None,
            dns_servers: None,
            ntp_servers: None,
            lease_ttl: None,
            boot_tftp: None,
        }
    }
}
