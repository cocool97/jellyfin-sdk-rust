use serde::Deserialize;

/// Structure received from `/System/ActivityLog/Entries` API calls.
#[derive(Deserialize, Debug)]
pub struct JellyfinLogEntry {
    #[serde(rename = "Id")]
    pub id: u16,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Overview")]
    pub overview: Option<String>,
    #[serde(rename = "ShortOverview")]
    pub short_overview: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "UserPrimaryImageTag")]
    pub user_primary_image_tag: Option<String>,
    #[serde(rename = "Severity")]
    pub severity: String,
}
