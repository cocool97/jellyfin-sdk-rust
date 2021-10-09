#[cfg(feature = "headers")]
use reqwest::header::HeaderMap;

use reqwest::{Response, Url};
use serde::de::DeserializeOwned;

use crate::{JellyfinSDKError, JellyfinSDKResult};

/// Structure containing the request status code and a `Result` that may contain the body.
pub struct JellyfinResponse<T> {
    pub(crate) status: u16,
    pub(crate) url: Url,
    #[cfg(feature = "headers")]
    pub(crate) headers: HeaderMap,
    pub(crate) body: Result<T, reqwest::Error>,
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
    {
        match response.error_for_status() {
            Ok(res) => Ok(JellyfinResponse {
                status: res.status().as_u16(),
                url: res.url().clone(),
                #[cfg(feature = "headers")]
                headers: res.headers().clone(),
                body: res.json::<T>().await,
            }),
            Err(err) => Err(JellyfinSDKError::ReqwestError(err)),
        }
    }
}
