#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlavourFlavour {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::ApiComputeResourceType>,
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<crate::models::FlavourFlavourAvailability>,
    #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<i32>,
    #[serde(rename = "cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "traffic", skip_serializing_if = "Option::is_none")]
    pub traffic: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::models::ApiPrice>,
    #[serde(rename = "windowsMonthlyFee", skip_serializing_if = "Option::is_none")]
    pub windows_monthly_fee: Option<crate::models::ApiPrice>,
    #[serde(rename = "pricePerMonth", skip_serializing_if = "Option::is_none")]
    pub price_per_month: Option<crate::models::ApiPrice>,
}

impl FlavourFlavour {
    pub fn new() -> FlavourFlavour {
        FlavourFlavour {
            id: None,
            name: None,
            _type: None,
            availability: None,
            generation: None,
            cpu: None,
            memory: None,
            disk: None,
            network: None,
            traffic: None,
            price: None,
            windows_monthly_fee: None,
            price_per_month: None,
        }
    }
}
