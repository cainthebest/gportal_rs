#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeRescueMode {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl ComputeRescueMode {
    pub fn new() -> ComputeRescueMode {
        ComputeRescueMode {
            enabled: None,
            password: None,
        }
    }
}
