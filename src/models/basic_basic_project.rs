#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicBasicProject {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl BasicBasicProject {
    pub fn new() -> BasicBasicProject {
        BasicBasicProject {
            id: None,
            name: None,
            avatar: None,
        }
    }
}
