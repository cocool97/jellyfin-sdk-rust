#[cfg(feature = "headers")]
use reqwest::header::HeaderMap;

use reqwest::{Response, Url};
use serde::de::DeserializeOwned;

use crate::{JellyfinSDKError, JellyfinSDKResult};

/// Structure containing the request status code and a `Result` that may contain the body.
pub struct JellyfinResponse<T> {
    status: u16,
    url: Url,
    #[cfg(feature = "headers")]
    headers: HeaderMap,
    body: Result<T, reqwest::Error>,
}

impl<T> JellyfinResponse<T> {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn body(&self) -> &Result<T, reqwest::Error> {
        &self.body
    }

    #[cfg(feature = "headers")]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub(crate) async fn async_from(response: Response) -> JellyfinSDKResult<Self>
    where
        T: DeserializeOwned,
        T: 'static,
    {
        let status = response.status().as_u16();

        if response.status().is_success() {
            Ok(JellyfinResponse {
                status,
                url: response.url().clone(),
                #[cfg(feature = "headers")]
                headers: response.headers().clone(),
                body: response.json::<T>().await,
            })
        } else {
            Err(JellyfinSDKError::HttpResponseError(status))
        }
    }
}
