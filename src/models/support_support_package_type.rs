///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportSupportPackageType {
    #[serde(rename = "BASIC_PLAN")]
    BASICPLAN,
    #[serde(rename = "ADVANCED_PLAN")]
    ADVANCEDPLAN,
}
