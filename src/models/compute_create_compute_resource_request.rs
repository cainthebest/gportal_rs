#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeCreateComputeResourceRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "flavourId", skip_serializing_if = "Option::is_none")]
    pub flavour_id: Option<String>,
    #[serde(rename = "datacenterId", skip_serializing_if = "Option::is_none")]
    pub datacenter_id: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "sshKeyIds", skip_serializing_if = "Option::is_none")]
    pub ssh_key_ids: Option<Vec<String>>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "userData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "networkIds", skip_serializing_if = "Option::is_none")]
    pub network_ids: Option<Vec<String>>,
    #[serde(rename = "billingPeriod", skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<crate::models::BillingBillingPeriod>,
}

impl ComputeCreateComputeResourceRequest {
    pub fn new() -> ComputeCreateComputeResourceRequest {
        ComputeCreateComputeResourceRequest {
            project_id: None,
            flavour_id: None,
            datacenter_id: None,
            password: None,
            ssh_key_ids: None,
            image_id: None,
            user_data: None,
            hosts: None,
            network_ids: None,
            billing_period: None,
        }
    }
}
