///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetadataMetadataInterfaceType {
    #[serde(rename = "PUBLIC")]
    PUBLIC,
    #[serde(rename = "PRIVATE")]
    PRIVATE,
}
