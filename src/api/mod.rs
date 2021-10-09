mod jellyfin_api;
pub use jellyfin_api::JellyfinAPI;

mod endpoints;

/// Module containing all Jellyfin API response types.
pub mod models;

mod url_builder;
pub(crate) use url_builder::URLBuilder;
