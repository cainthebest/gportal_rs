#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageOperatingSystem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl ImageOperatingSystem {
    pub fn new() -> ImageOperatingSystem {
        ImageOperatingSystem {
            id: None,
            name: None,
            icon: None,
        }
    }
}
