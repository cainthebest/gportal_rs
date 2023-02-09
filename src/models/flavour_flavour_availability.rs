///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlavourFlavourAvailability {
    #[serde(rename = "OUT_OF_STOCK")]
    OUTOFSTOCK,
    #[serde(rename = "PREORDER")]
    PREORDER,
    #[serde(rename = "LOW")]
    LOW,
    #[serde(rename = "MID")]
    MID,
    #[serde(rename = "HIGH")]
    HIGH,
}
