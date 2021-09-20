use reqwest::header::HeaderMap;

/// Structure managing all API interactions.
pub struct JellyfinAPI {
    async_client: reqwest::Client,
    #[cfg(feature = "sync")]
    sync_client: reqwest::blocking::Client,
    base_addr: String,
}

impl JellyfinAPI {
    pub(crate) fn new<S: Into<String>>(base_addr: S, token: Option<&str>) -> Self {
        static USER_AGENT: &str = concat!("Jellyfin SDK - ", env!("CARGO_PKG_VERSION"));
        // TODO
        // - Version to get dynamically
        // - Test unauthenticated routes

        let mut headers = HeaderMap::new();

        match token {
            Some(t) => {
                headers.insert("X-Emby-Token", t.parse().unwrap());
            }
            None => {
                headers.insert(
                    "X-Emby-Authorization",
                    r#"MediaBrowser Client="toto", Version="10.7.7"#.parse().unwrap(),
                );
            }
        }

        let async_client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .unwrap();
        #[cfg(feature = "sync")]
        let sync_client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            async_client,
            #[cfg(feature = "sync")]
            sync_client,
            base_addr: base_addr.into(),
        }
    }

    pub(crate) fn async_client(&self) -> &reqwest::Client {
        &self.async_client
    }

    #[cfg(feature = "sync")]
    pub(crate) fn sync_client(&self) -> &reqwest::blocking::Client {
        &self.sync_client
    }

    pub(crate) fn base_addr(&self) -> &String {
        &self.base_addr
    }
}
