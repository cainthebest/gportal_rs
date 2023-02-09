#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportSupportPackage {
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<crate::models::SupportSupportPackageType>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::models::ApiPrice>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "support", skip_serializing_if = "Option::is_none")]
    pub support: Option<crate::models::SupportSupportPackageSupport>,
}

impl SupportSupportPackage {
    pub fn new() -> SupportSupportPackage {
        SupportSupportPackage {
            plan: None,
            name: None,
            price: None,
            active: None,
            support: None,
        }
    }
}
