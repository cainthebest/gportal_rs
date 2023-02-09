///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserWebAuthnAuthenticator {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "TOUCH_ID")]
    TOUCHID,
    #[serde(rename = "YUBIKEY")]
    YUBIKEY,
}
