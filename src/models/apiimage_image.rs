#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiimageImage {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "operatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<crate::models::ImageOperatingSystem>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(
        rename = "authenticationTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_types: Option<Vec<crate::models::ImageAuthenticationType>>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::ProjectProject>,
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::ImageImageVersion>>,
    #[serde(rename = "imageUpload", skip_serializing_if = "Option::is_none")]
    pub image_upload: Option<crate::models::ImageImageUpload>,
    #[serde(rename = "vendorData", skip_serializing_if = "Option::is_none")]
    pub vendor_data: Option<String>,
    #[serde(rename = "splaLicense", skip_serializing_if = "Option::is_none")]
    pub spla_license: Option<bool>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ApiimageImage {
    pub fn new() -> ApiimageImage {
        ApiimageImage {
            id: None,
            operating_system: None,
            name: None,
            username: None,
            public: None,
            available: None,
            authentication_types: None,
            project: None,
            versions: None,
            image_upload: None,
            vendor_data: None,
            spla_license: None,
            created_at: None,
            updated_at: None,
        }
    }
}
