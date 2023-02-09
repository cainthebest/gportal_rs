#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportListProjectSupportPackagesResponse {
    #[serde(rename = "packages", skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<crate::models::SupportSupportPackage>>,
}

impl SupportListProjectSupportPackagesResponse {
    pub fn new() -> SupportListProjectSupportPackagesResponse {
        SupportListProjectSupportPackagesResponse { packages: None }
    }
}
