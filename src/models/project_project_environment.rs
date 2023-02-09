///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectProjectEnvironment {
    #[serde(rename = "DEVELOPMENT")]
    DEVELOPMENT,
    #[serde(rename = "STAGING")]
    STAGING,
    #[serde(rename = "PRODUCTION")]
    PRODUCTION,
}
