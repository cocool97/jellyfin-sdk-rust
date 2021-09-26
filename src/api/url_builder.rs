use std::fmt::Display;

use url::Url;

use crate::{JellyfinSDKError, JellyfinSDKResult};

pub struct URLBuilder {
    url: Url,
}

impl URLBuilder {
    pub fn from(base_addr: &str, path: &str) -> JellyfinSDKResult<Self> {
        let mut url = Url::parse(base_addr)?;

        let path = if path.ends_with('/') {
            // Removes trailing '/' if present...
            Self::remove_trailing_slash(path)
        } else {
            path
        };

        url.set_path(path);

        Ok(Self { url })
    }

    pub fn remove_trailing_slash(path: &str) -> &str {
        &path[0..path.len() - 1]
    }

    pub fn join_path(&mut self, path: &str) -> JellyfinSDKResult<()> {
        self.url
            .path_segments_mut()
            .map_err(|_| JellyfinSDKError::CannotGetPathSegments)?
            .push(Self::remove_trailing_slash(path));

        Ok(())
    }

    pub fn add_param<S: Display>(&mut self, param: &'static str, value: S) {
        self.url
            .query_pairs_mut()
            .append_pair(param, &value.to_string());
    }

    pub fn add_optional_param<S: Display>(&mut self, param: &'static str, value: Option<S>) {
        if let Some(value) = value {
            self.add_param(param, value);
        }
    }

    pub fn build(self) -> String {
        self.url.to_string()
    }
}
