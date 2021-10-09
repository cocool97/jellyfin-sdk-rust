use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SystemEndpointStatus {
    #[serde(rename = "IsLocal")]
    pub is_local: bool,
    #[serde(rename = "IsInNetwork")]
    pub is_in_network: bool,
}

#[derive(Deserialize, Debug)]
pub struct SystemInformations {
    #[serde(rename = "LocalAddress")]
    pub local_address: String,
    #[serde(rename = "ServerName")]
    pub server_name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "StartupWizardCompleted")]
    pub startup_wizard_complete: Option<bool>,
    #[serde(rename = "OperatingSystemDisplayName")]
    pub operating_system: String,
    #[serde(rename = "PackageName")]
    pub package_name: Option<String>,
    #[serde(rename = "HasPendingRestart")]
    pub has_pending_restart: bool,
    #[serde(rename = "IsShuttingDown")]
    pub is_shutting_down: bool,
    #[serde(rename = "SupportsLibraryMonitor")]
    pub supports_library_monitor: bool,
    #[serde(rename = "WebSocketPortNumber")]
    pub websocket_port: u16,
    #[serde(rename = "CompletedInstallations")]
    pub completed_installations: Vec<CompletedInstallations>,
    #[serde(rename = "CanSelfRestart")]
    pub can_self_restart: bool,
    #[serde(rename = "CanLaunchWebBrowser")]
    pub can_launch_webbrowser: bool,
    #[serde(rename = "ProgramDataPath")]
    pub program_data_path: String,
    #[serde(rename = "WebPath")]
    pub web_path: String,
    #[serde(rename = "ItemsByNamePath")]
    pub items_by_name_path: String,
    #[serde(rename = "CachePath")]
    pub cache_path: String,
    #[serde(rename = "LogPath")]
    pub log_path: String,
    #[serde(rename = "InternalMetadataPath")]
    pub internal_metadata_path: String,
    #[serde(rename = "TranscodingTempPath")]
    pub transcoding_temp_path: String,
    #[serde(rename = "HasUpdateAvailable")]
    pub has_update_available: bool,
    #[serde(rename = "EncoderLocation")]
    pub encoder_location: String,
    #[serde(rename = "SystemArchitecture")]
    pub system_arch: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletedInstallations {
    #[serde(rename = "Guid")]
    pub guid: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: Version,
    #[serde(rename = "Changelog")]
    pub changelog: String,
    #[serde(rename = "SourceUrl")]
    pub source_url: String,
    #[serde(rename = "Checksum")]
    pub checksum: String,
    #[serde(rename = "PackageInfo")]
    pub package_info: PackageInfo,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "Major")]
    pub major: u8,
    #[serde(rename = "Minor")]
    pub minor: u8,
    #[serde(rename = "Build")]
    pub build: u16,
    #[serde(rename = "Revision")]
    pub revision: u16,
    #[serde(rename = "MajorRevision")]
    pub major_revision: u16,
    #[serde(rename = "MinorRevision")]
    pub minor_revision: u16,
}

#[derive(Deserialize, Debug)]
pub struct PackageInfo {
    pub name: String,
    pub description: String,
    pub overview: String,
    pub owner: String,
    pub category: String,
    pub guid: String,
    pub versions: Vec<PackageVersion>,
    pub image_url: String,
}

#[derive(Deserialize, Debug)]
pub struct PackageVersion {
    pub version: String,
    pub version_number: Version,
    pub changelog: String,
    pub target_abi: String,
    pub source_url: String,
    pub checksum: String,
    pub timestamp: String,
    pub repository_name: String,
    pub repository_url: String,
}

#[derive(Deserialize, Debug)]
pub struct PublicSystemInformations {
    #[serde(rename = "LocalAddress")]
    pub local_address: String,
    #[serde(rename = "ServerName")]
    pub server_name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "ProductName")]
    pub product_name: String,
    #[serde(rename = "OperatingSystem")]
    pub operating_system: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "StartupWizardCompleted")]
    pub startup_wizard_completed: bool,
}

pub type AvailableLogFiles = Vec<AvailableLogFile>;

#[derive(Deserialize, Debug)]
pub struct AvailableLogFile {
    #[serde(rename = "DateCreated")]
    date_created: String,
    #[serde(rename = "DateModified")]
    date_modified: String,
    #[serde(rename = "Size")]
    size: u64,
    #[serde(rename = "Name")]
    name: String,
}

pub type WakeOnLanInformations = Vec<WakeOnLanInformation>;

#[derive(Deserialize, Debug)]
pub struct WakeOnLanInformation {
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "Port")]
    pub port: u32,
}
