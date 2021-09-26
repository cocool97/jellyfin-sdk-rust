use crate::{
    models::system::{PublicSystemInformations, SystemEndpointStatus, SystemInformations},
    JellyfinAPI, JellyfinResponse, JellyfinSDKResult,
};

use super::URLBuilder;

impl JellyfinAPI {
    #[cfg(feature = "sync")]
    pub fn get_endpoint_informations() {}

    /// Gets information about the request endpoint.
    pub async fn async_get_endpoint_informations(
        &self,
    ) -> JellyfinSDKResult<JellyfinResponse<SystemEndpointStatus>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Endpoint")?;

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn get_system_informations() {}

    /// Gets information about the server.
    pub async fn async_get_system_informations(
        &self,
    ) -> JellyfinSDKResult<JellyfinResponse<SystemInformations>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Info")?;

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn get_public_system_informations() {}

    /// Gets public information about the server.
    pub async fn async_get_public_system_informations(
        &self,
    ) -> JellyfinSDKResult<JellyfinResponse<PublicSystemInformations>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Info/Public")?;

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }
}
