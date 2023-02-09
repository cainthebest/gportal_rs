#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectInviteMemberToProjectResponse {
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<crate::models::ProjectProjectMember>,
}

impl ProjectInviteMemberToProjectResponse {
    pub fn new() -> ProjectInviteMemberToProjectResponse {
        ProjectInviteMemberToProjectResponse { member: None }
    }
}
