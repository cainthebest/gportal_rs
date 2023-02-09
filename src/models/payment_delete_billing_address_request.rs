#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentDeleteBillingAddressRequest {
    #[serde(rename = "addressId", skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
}

impl PaymentDeleteBillingAddressRequest {
    pub fn new() -> PaymentDeleteBillingAddressRequest {
        PaymentDeleteBillingAddressRequest { address_id: None }
    }
}
