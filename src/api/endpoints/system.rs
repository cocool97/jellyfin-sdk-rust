use crate::{
    api::URLBuilder,
    models::system::{
        AvailableLogFiles, PublicSystemInformations, SystemEndpointStatus, SystemInformations,
        WakeOnLanInformations,
    },
    JellyfinAPI, JellyfinResponse, JellyfinSDKResult,
};

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

    #[cfg(feature = "sync")]
    pub fn get_available_log_files() {}

    /// Gets a list of available server log files.
    pub async fn async_get_available_log_files(
        &self,
    ) -> JellyfinSDKResult<JellyfinResponse<AvailableLogFiles>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Logs")?;

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn get_log_file() {}

    /// Gets a log file.
    pub async fn async_get_log_file(
        &self,
        name: &str,
    ) -> JellyfinSDKResult<JellyfinResponse<String>> {
        let mut url = URLBuilder::from(self.base_addr(), "/System/Logs/Log")?;
        url.add_param("name", name);

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn ping_server_http_get() {}

    pub async fn async_ping_server_http_get(&self) -> JellyfinSDKResult<JellyfinResponse<String>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Ping")?;

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn ping_server_http_post() {}

    pub async fn async_ping_server_http_post(&self) -> JellyfinSDKResult<JellyfinResponse<String>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Ping")?;

        let res = self.async_client().post(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn restart_server() {}

    pub async fn async_restart_server(&self) -> JellyfinSDKResult<JellyfinResponse<()>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Restart")?;

        let res = self.async_client().post(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn shutdown_server() {}

    pub async fn async_shutdown_server(&self) -> JellyfinSDKResult<JellyfinResponse<()>> {
        let url = URLBuilder::from(self.base_addr(), "/System/Shutdown")?;

        let res = self.async_client().post(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }

    #[cfg(feature = "sync")]
    pub fn get_wake_on_lan_informations() {}

    pub async fn async_get_wake_on_lan_informations(
        &self,
    ) -> JellyfinSDKResult<JellyfinResponse<WakeOnLanInformations>> {
        let url = URLBuilder::from(self.base_addr(), "/System/WakeOnLanInfo")?;

        let res = self.async_client().get(url.build()).send().await?;

        JellyfinResponse::async_from(res).await
    }
}
