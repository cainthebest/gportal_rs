#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionRegion {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "datacenters", skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<Vec<crate::models::RegionDataCenter>>,
}

impl RegionRegion {
    pub fn new() -> RegionRegion {
        RegionRegion {
            id: None,
            name: None,
            country_code: None,
            datacenters: None,
        }
    }
}
