use crate::{models::keys::JellyfinKey, JellyfinAPI, JellyfinResponse, JellyfinSDKResult};

impl JellyfinAPI {
    #[cfg(feature = "sync")]
    pub fn get_all_keys(&self) {
        unimplemented!()
    }

    pub async fn async_get_all_keys(&self) -> JellyfinSDKResult<JellyfinResponse<JellyfinKey>> {
        let url = format!("{}/Auth/Keys", self.base_addr());

        let res = self.async_client().get(url).send().await?;

        Ok(JellyfinResponse::async_from(res).await)
    }
}
