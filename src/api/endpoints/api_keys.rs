use crate::{
    api::URLBuilder,
    models::{keys::JellyfinKey, JellyfinBaseResponse},
    JellyfinAPI, JellyfinResponse, JellyfinSDKResult,
};

impl JellyfinAPI {
    #[cfg(feature = "sync")]
    pub fn get_all_keys(&self) {
        unimplemented!()
    }

    /// Get all keys.
    pub async fn async_get_keys(
        &self,
    ) -> JellyfinSDKResult<JellyfinResponse<JellyfinBaseResponse<JellyfinKey>>> {
        let url = URLBuilder::from(self.base_addr(), "/Auth/Keys")?;

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn create_key(&self, app: &str) {}

    /// Create a new api key.
    pub async fn async_create_key(&self, app: &str) -> JellyfinSDKResult<JellyfinResponse<()>> {
        let mut url = URLBuilder::from(self.base_addr(), "/Auth/Keys")?;
        url.add_param("App", app);

        let res = self.async_client().post(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn revoke_key(&self, key: &str) {}

    /// Remove an api key.
    pub async fn async_revoke_key(&self, key: &str) -> JellyfinSDKResult<JellyfinResponse<()>> {
        let mut url = URLBuilder::from(self.base_addr(), "/Auth/Keys/")?;
        url.join_path(key)?;

        let res = self.async_client().delete(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }
}
