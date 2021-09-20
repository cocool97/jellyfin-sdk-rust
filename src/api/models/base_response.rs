use serde::Deserialize;

/// Base structure received with all Jellyfin API calls.
///
/// This structure wraps all underlying structures.
#[derive(Deserialize, Debug)]
pub struct JellyfinBaseResponse<T> {
    #[serde(rename = "Items")]
    pub items: Vec<T>,
    #[serde(rename = "TotalRecordCount")]
    pub total_record_count: u16,
    #[serde(rename = "StartIndex")]
    pub start_index: u16,
}
