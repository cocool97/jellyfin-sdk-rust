use serde::Deserialize;

/// Structure received from `/Auth/Keys` API calls.
#[derive(Deserialize, Debug)]
pub struct JellyfinKey {
    #[serde(rename = "Id")]
    pub id: u16,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "AppName")]
    pub app_name: String,
    #[serde(rename = "AppVersion")]
    pub app_version: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "DateRevoked")]
    pub date_revoked: Option<String>,
    #[serde(rename = "DateLastActivity")]
    pub date_last_activity: String,
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}
