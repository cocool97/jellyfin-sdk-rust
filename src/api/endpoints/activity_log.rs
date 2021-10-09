use crate::api::models::logs::JellyfinLogEntry;
use crate::api::URLBuilder;
use crate::models::JellyfinBaseResponse;
use crate::{JellyfinAPI, JellyfinResponse, JellyfinSDKResult};

impl JellyfinAPI {
    #[cfg(feature = "sync")]
    pub fn get_log_entries(&self) -> JellyfinSDKResult<()> {
        unimplemented!()
    }

    /// Gets activity log entries.
    pub async fn async_get_log_entries(
        &self,
        start_index: Option<u32>,
        limit: Option<u32>,
        min_date: Option<String>,
        has_user_id: Option<bool>,
    ) -> JellyfinSDKResult<JellyfinResponse<JellyfinBaseResponse<JellyfinLogEntry>>> {
        let mut url = URLBuilder::from(self.base_addr(), "/System/ActivityLog/Entries")?;

        url.add_optional_param("startIndex", start_index);
        url.add_optional_param("limit", limit);
        url.add_optional_param("minDate", min_date);
        url.add_optional_param("hasUserId", has_user_id);

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }
}
