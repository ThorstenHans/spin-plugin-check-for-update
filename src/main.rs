#![warn(rust_2018_idioms)]

use anyhow::Result;
use serde::Deserialize;
use spinners::{Spinner, Spinners};

// link printed when local installation is outdated
const SPIN_INSTALL_INSTRUCTIONS: &str = "https://developer.fermyon.com/spin/install";
// root url for retrieving the latest spin-cli version
const GET_LATEST_SPIN_CLI_ROOT_URL: &str =
    "https://get-latest-spin-cli-version-wuvznxqk.fermyon.app/version";

// Spin provides the currently installed version of the spin-cli via environment variable
const SPIN_CLI_VERSION_ENV: &str = "SPIN_VERSION";
// Spin provides the currently installed commit sha of the spin-cli via environment variable
const SPIN_CLI_COMMIT_SHA_ENV: &str = "SPIN_COMMIT_SHA";

#[derive(Debug, Deserialize)]
pub struct SpinCliVersion {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "commit_hash")]
    pub commit_sha: String,
}

impl std::fmt::Display for SpinCliVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.version, self.commit_sha)
    }
}
impl SpinCliVersion {
    fn is_canary(&self) -> bool {
        self.version.contains("pre")
    }

    fn is_outdated(&self, other: &SpinCliVersion) -> bool {
        if self.is_canary() {
            return self.commit_sha != other.commit_sha;
        }
        // do it that way because latest stable release on GitHub has commit sha set to "main"
        // however, I'm uncertain what the SPIN_COMMIT_SHA env var is set to in the plugin for stable releases
        self.version != other.version
    }
}

fn get_installed_spin_version() -> Result<SpinCliVersion> {
    let version = std::env::var(SPIN_CLI_VERSION_ENV)?;
    let commit_sha = std::env::var(SPIN_CLI_COMMIT_SHA_ENV)?;
    Ok(SpinCliVersion {
        version,
        commit_sha,
    })
}

fn get_latest_spin_release(canary: bool) -> Result<SpinCliVersion> {
    let url = match canary {
        true => format!("{}/canary", GET_LATEST_SPIN_CLI_ROOT_URL),
        false => format!("{}/stable", GET_LATEST_SPIN_CLI_ROOT_URL),
    };

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;

    response
        .json::<SpinCliVersion>()
        .map_err(anyhow::Error::msg)
}

fn main() {
    let mut spinner = Spinner::new(
        Spinners::Dots12,
        "Checking for latest spin CLI version...".into(),
    );

    let Ok(installed) = get_installed_spin_version() else {
        println!("Failed to get installed version of spin-cli");
        return;
    };

    let Ok(latest) = get_latest_spin_release(installed.is_canary()) else {
        println!("Failed to get latest version of spin-cli");
        return;
    };

    spinner.stop_with_newline();

    println!();

    if !installed.is_outdated(&latest) {
        println!("Your spin CLI is up to date! {}) ✅", installed);
    } else {
        println!("Installed spin CLI version:   {}", installed);
        println!("Latest spin CLI version:      {}", latest);
        println!();
        println!(
            "See instructions for updating your spin CLI installation at {}",
            SPIN_INSTALL_INSTRUCTIONS
        );
    }

    if has_homebrew() {
        println!();
        println!("You can also install and mange your spin CLI with Homebrew:");
        println!("    brew tap fermyon/tap");
        println!("    brew install fermyon/tap/spin");
    }
}

fn has_homebrew() -> bool {
    if !cfg!(target_os = "macos") && !cfg!(target_os = "linux") {
        return false;
    }
    let output = std::process::Command::new("which")
        .arg("brew")
        .output()
        .expect("Failed to check for homebrew");
    output.status.success()
}
