#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RpcStatus {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::ProtobufAny>>,
}

impl RpcStatus {
    pub fn new() -> RpcStatus {
        RpcStatus {
            code: None,
            message: None,
            details: None,
        }
    }
}
