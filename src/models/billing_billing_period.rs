///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingBillingPeriod {
    #[serde(rename = "BILLING_PERIOD_HOURLY")]
    HOURLY,
    #[serde(rename = "BILLING_PERIOD_MONTHLY")]
    MONTHLY,
}
