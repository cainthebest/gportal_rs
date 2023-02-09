///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectMemberRole {
    #[serde(rename = "MEMBER")]
    MEMBER,
    #[serde(rename = "ADMIN")]
    ADMIN,
    #[serde(rename = "OWNER")]
    OWNER,
}
