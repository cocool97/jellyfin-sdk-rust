use reqwest::{header::HeaderMap, Response};
use serde::de::DeserializeOwned;

use crate::models::JellyfinBaseResponse;

/// Structure containing the request status code and a `Result` that may contain the body.
pub struct JellyfinResponse<T> {
    status: u16,
    headers: HeaderMap,
    body: Result<JellyfinBaseResponse<T>, reqwest::Error>,
}

impl<T> JellyfinResponse<T> {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &Result<JellyfinBaseResponse<T>, reqwest::Error> {
        &self.body
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub(crate) async fn async_from(response: Response) -> Self
    where
        T: DeserializeOwned,
        T: 'static,
    {
        JellyfinResponse {
            status: response.status().as_u16(),
            headers: response.headers().clone(),
            body: response.json::<JellyfinBaseResponse<T>>().await,
        }
    }
}
