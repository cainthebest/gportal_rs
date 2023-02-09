///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CidrVersion {
    #[serde(rename = "IPV4")]
    IPV4,
    #[serde(rename = "IPV6")]
    IPV6,
}
