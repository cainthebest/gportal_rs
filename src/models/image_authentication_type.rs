///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageAuthenticationType {
    #[serde(rename = "PASSWORD")]
    PASSWORD,
    #[serde(rename = "SSH_KEY")]
    SSHKEY,
}
