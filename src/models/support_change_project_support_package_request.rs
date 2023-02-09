#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportChangeProjectSupportPackageRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<crate::models::SupportSupportPackageType>,
}

impl SupportChangeProjectSupportPackageRequest {
    pub fn new() -> SupportChangeProjectSupportPackageRequest {
        SupportChangeProjectSupportPackageRequest {
            project_id: None,
            plan: None,
        }
    }
}
