#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentListBillingAddressesResponse {
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<crate::models::PaymentBillingAddress>>,
}

impl PaymentListBillingAddressesResponse {
    pub fn new() -> PaymentListBillingAddressesResponse {
        PaymentListBillingAddressesResponse { addresses: None }
    }
}
