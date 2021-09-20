use crate::{JellyfinAPI, JellyfinSDKError, JellyfinSDKResult};

/// Main structure representing the Jellyfin instance.
///
/// Must be used to interact with Jellyfin server across different ways:
///
/// - API calls
/// - Servers discovery
/// - WebSockets calls
pub struct JellyfinSDK {
    /// Handles interactions with Jellyfin API.
    api: Option<JellyfinAPI>,
    // Discovers new Jellyfin servers.
    // discovery: Option<JellyfinDiscovery>,
    // Handles interactions with Jellyfin via Web sockets.
    // websocket: Option<JellyfinWebSocket>,
    // Holds all client informations
    // client_infos: Option<ClientInfos>,
}

impl JellyfinSDK {
    /// Instantiates a new `JellyfinSDK` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `JellyfinAPI` instance.
    pub fn create_api<S: Into<String>>(&mut self, base_addr: S, token: Option<&str>) {
        self.api = Some(JellyfinAPI::new(base_addr, token))
    }

    /// Tries getting the [`JellyfinAPI`] instance.
    ///
    /// Returns an [`JellyfinSDKError`] if no [`JellyfinAPI`] has been initialized before calling this method.
    pub fn get_api(&self) -> JellyfinSDKResult<&JellyfinAPI> {
        self.api
            .as_ref()
            .ok_or_else(|| JellyfinSDKError::NoJellyfinAPICreated)
    }
}

impl Default for JellyfinSDK {
    fn default() -> Self {
        JellyfinSDK {
            api: None,
            // discovery: None,
            // websocket: None,
            // client_infos: None,
        }
    }
}
