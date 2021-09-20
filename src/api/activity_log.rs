use crate::api::models::logs::JellyfinLogEntry;
use crate::{JellyfinAPI, JellyfinResponse, JellyfinSDKResult};

impl JellyfinAPI {
    #[cfg(feature = "sync")]
    pub fn get_log_entries(&self) -> JellyfinSDKResult<()> {
        unimplemented!()
    }

    pub async fn async_get_log_entries(
        &self,
        start_index: Option<u32>,
        limit: Option<u32>,
        min_date: Option<String>,
        has_user_id: Option<bool>,
    ) -> JellyfinSDKResult<JellyfinResponse<JellyfinLogEntry>> {
        let mut url = format!("{}/System/ActivityLog/Entries", self.base_addr());

        if let Some(s) = start_index {
            url.push_str(&format!("?startIndex={}", s.to_string()));
        }

        if let Some(l) = limit {
            url.push_str(&format!("?limit={}", l.to_string()));
        }

        if let Some(m) = min_date {
            url.push_str(&format!("?minDate={}", m));
        }

        if let Some(h) = has_user_id {
            url.push_str(&format!("?hasUserId={}", h.to_string()));
        }

        let res = self.async_client().get(url).send().await?;

        Ok(JellyfinResponse::async_from(res).await)
    }
}
