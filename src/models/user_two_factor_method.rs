///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserTwoFactorMethod {
    #[serde(rename = "RECOVERY_CODES")]
    RECOVERYCODES,
    #[serde(rename = "TOTP")]
    TOTP,
    #[serde(rename = "FIDO2")]
    FIDO2,
}
