///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingProduct {
    #[serde(rename = "CREDIT")]
    CREDIT,
    #[serde(rename = "VOUCHER")]
    VOUCHER,
    #[serde(rename = "COMPUTE")]
    COMPUTE,
    #[serde(rename = "WINDOWS_LICENSE")]
    WINDOWSLICENSE,
    #[serde(rename = "SUPPORT")]
    SUPPORT,
    #[serde(rename = "TRAFFIC")]
    TRAFFIC,
}
