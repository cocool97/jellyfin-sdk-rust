use thiserror::Error;

/// Custom Result type to handle SDK errors.
pub type JellyfinSDKResult<T> = std::result::Result<T, JellyfinSDKError>;

/// Enum representing all possible SDK errors.
#[derive(Error, Debug)]
pub enum JellyfinSDKError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidURL(#[from] url::ParseError),
    #[error("Cannot get path segments...")]
    CannotGetPathSegments,
    #[error("No Jellyfin instance has been created before calling `get_api()` method...")]
    NoJellyfinAPICreated,
    #[error("Received status code - {0}")]
    HttpResponseError(u16),
}
