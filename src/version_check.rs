//! Module for checking crate version against the latest published version on crates.io

use std::time::Duration;

use serde::Deserialize;

use crate::crate_info;

/// Response structure from crates.io API
#[derive(Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
}

/// Crate information from crates.io API
#[derive(Deserialize)]
struct CrateInfo {
    /// The latest stable version published on crates.io
    max_stable_version: Option<semver::Version>,
}

/// Fetches the latest published version from crates.io
///
/// Returns `Some(version)` with the latest stable version published on crates.io,
/// or `None` if no stable version has been published yet.
///
/// # Errors
///
/// Returns an error if:
/// - Network request fails or times out
/// - The crates.io API response cannot be parsed
///
/// # Example
/// ```no_run
/// # use shell_cell::version_check::latest_published_version;
/// # use shell_cell::crate_info;
/// #
/// # async fn example() -> color_eyre::Result<()> {
/// if let Some(latest) = latest_published_version().await? {
///     let current = semver::Version::parse(crate_info::version())?;
///     if latest > current {
///         println!(
///             "A new version {} is available! (current: {})",
///             latest, current
///         );
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub async fn latest_published_version() -> color_eyre::Result<Option<semver::Version>> {
    const CRATES_IO_API: &str = "https://crates.io/api/v1/crates";
    const REQUEST_TIMEOUT_SECS: Duration = Duration::from_secs(5);

    // Construct API URL
    let url = format!("{CRATES_IO_API}/{}", crate_info::name());

    // Create HTTP client with timeout
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT_SECS)
        .user_agent(crate_info::name())
        .build()?;

    // Make HTTP request
    let response = client.get(&url).send().await?;

    // Parse JSON response
    let crates_info: CratesIoResponse = response.json().await?;

    // Return the published version
    Ok(crates_info.crate_info.max_stable_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_latest_published_version() {
        // This test verifies the function executes without panicking
        // The result may be Ok(Some(version)), Ok(None), or Err depending on network availability
        let _result = latest_published_version().await;
    }
}
