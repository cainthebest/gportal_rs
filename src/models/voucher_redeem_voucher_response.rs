#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoucherRedeemVoucherResponse {
    #[serde(rename = "credit", skip_serializing_if = "Option::is_none")]
    pub credit: Option<crate::models::ApiPrice>,
}

impl VoucherRedeemVoucherResponse {
    pub fn new() -> VoucherRedeemVoucherResponse {
        VoucherRedeemVoucherResponse { credit: None }
    }
}
