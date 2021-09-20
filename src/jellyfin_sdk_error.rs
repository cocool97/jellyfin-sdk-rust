use thiserror::Error;

/// Error type containing either the parsed structure or an error.
pub type JellyfinSDKResult<T> = std::result::Result<T, JellyfinSDKError>;

#[derive(Error, Debug)]
pub enum JellyfinSDKError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("No Jellyfin instance has been created before calling `get_api()` method...")]
    NoJellyfinAPICreated,
}
