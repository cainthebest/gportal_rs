#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeReinstallComputeResourceRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "fqdn", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "sshKeyIds", skip_serializing_if = "Option::is_none")]
    pub ssh_key_ids: Option<Vec<String>>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "userData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

impl ComputeReinstallComputeResourceRequest {
    pub fn new() -> ComputeReinstallComputeResourceRequest {
        ComputeReinstallComputeResourceRequest {
            project_id: None,
            resource_id: None,
            fqdn: None,
            password: None,
            ssh_key_ids: None,
            image_id: None,
            user_data: None,
        }
    }
}
