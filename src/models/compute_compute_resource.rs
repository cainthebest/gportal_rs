#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeComputeResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "ips", skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<crate::models::ComputeComputeResourceIp>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ApiComputeResourceType>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ComputeComputeResourceStatus>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<crate::models::RegionDataCenter>,
    #[serde(rename = "rescueMode", skip_serializing_if = "Option::is_none")]
    pub rescue_mode: Option<crate::models::ComputeRescueMode>,
    #[serde(rename = "flavour", skip_serializing_if = "Option::is_none")]
    pub flavour: Option<crate::models::FlavourFlavour>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<crate::models::ApiimageImage>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::models::ApiPrice>,
    #[serde(rename = "splaLicense", skip_serializing_if = "Option::is_none")]
    pub spla_license: Option<bool>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<crate::models::ServerServer>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "jobStatus", skip_serializing_if = "Option::is_none")]
    pub job_status: Option<crate::models::ComputeComputeResourceJobStatus>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "hasConsoleAccess", skip_serializing_if = "Option::is_none")]
    pub has_console_access: Option<bool>,
    #[serde(rename = "billingPeriod", skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<crate::models::BillingBillingPeriod>,
    #[serde(rename = "paidUntil", skip_serializing_if = "Option::is_none")]
    pub paid_until: Option<String>,
    #[serde(rename = "renewState", skip_serializing_if = "Option::is_none")]
    pub renew_state: Option<crate::models::ComputeComputeResourceRenewState>,
}

impl ComputeComputeResource {
    pub fn new() -> ComputeComputeResource {
        ComputeComputeResource {
            id: None,
            fqdn: None,
            ips: None,
            _type: None,
            status: None,
            tags: None,
            datacenter: None,
            rescue_mode: None,
            flavour: None,
            image: None,
            price: None,
            spla_license: None,
            server: None,
            created_at: None,
            job_status: None,
            project_id: None,
            has_console_access: None,
            billing_period: None,
            paid_until: None,
            renew_state: None,
        }
    }
}
