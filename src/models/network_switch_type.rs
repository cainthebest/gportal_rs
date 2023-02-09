///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkSwitchType {
    #[serde(rename = "Juniper")]
    Juniper,
    #[serde(rename = "JuniperLegacy")]
    JuniperLegacy,
}
