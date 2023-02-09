#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserListUserSshKeysResponse {
    #[serde(rename = "sshKeys", skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<crate::models::SecuritySshKey>>,
}

impl UserListUserSshKeysResponse {
    pub fn new() -> UserListUserSshKeysResponse {
        UserListUserSshKeysResponse { ssh_keys: None }
    }
}
