#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkListSwitchesResponse {
    #[serde(rename = "switches", skip_serializing_if = "Option::is_none")]
    pub switches: Option<Vec<crate::models::NetworkSwitch>>,
}

impl NetworkListSwitchesResponse {
    pub fn new() -> NetworkListSwitchesResponse {
        NetworkListSwitchesResponse { switches: None }
    }
}
