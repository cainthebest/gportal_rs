#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeGetComputeResourceConsoleResponse {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl ComputeGetComputeResourceConsoleResponse {
    pub fn new() -> ComputeGetComputeResourceConsoleResponse {
        ComputeGetComputeResourceConsoleResponse {
            token: None,
            password: None,
        }
    }
}
