use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct DesktopEntryIni {
    #[serde(rename = "Desktop Entry")]
    pub content: DesktopEntryInContent,
}

#[derive(Debug, Deserialize)]
pub struct DesktopEntryInContent {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Exec")]
    pub exec: String,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Keywords")]
    pub keywords: Option<String>,
}
