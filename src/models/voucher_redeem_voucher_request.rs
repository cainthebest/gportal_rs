#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherRedeemVoucherRequest {
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl VoucherRedeemVoucherRequest {
    pub fn new() -> VoucherRedeemVoucherRequest {
        VoucherRedeemVoucherRequest {
            project_id: None,
            code: None,
        }
    }
}
