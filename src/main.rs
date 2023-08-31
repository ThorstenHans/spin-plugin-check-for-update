#![warn(rust_2018_idioms)]
use anyhow::Result;
use spinners::{Spinner, Spinners};

const SPIN_INSTALL_INSTRUCTIONS: &str = "https://developer.fermyon.com/spin/install";
const GET_LATEST_SPIN_CLI_URL: &str= "https://get-latest-spin-cli-version-wuvznxqk.fermyon.app/version";
const SPIN_CLI_VERSION_ENV: &str = "SPIN_VERSION";
fn main() {
    let mut spinner = Spinner::new(Spinners::Dots12, "Checking for latest spin CLI version...".into());

    let Ok(installed) = get_installed_spin_cli_version() else {
        println!("Failed to get installed version of spin-cli");
        return;
    };
    
    let Ok(latest) = get_latest_spin_cli_version() else {
        println!("Failed to get latest version of spin-cli");
        return;
    };
    spinner.stop_with_newline();
    println!();
    if latest == installed {
        println!("Your spin CLI is up to date (version {}) ✅", installed);
    } else {
        println!("Installed spin CLI version:   {}", installed);
        println!("Latest spin CLI version:      {}", latest);
        println!();
        println!("See instructions for updating your spin CLI installation at {}", SPIN_INSTALL_INSTRUCTIONS);
    }
    
}

fn get_installed_spin_cli_version() -> Result<String> {
    let current = std::env::var(SPIN_CLI_VERSION_ENV)?;
    Ok(current)
}

fn get_latest_spin_cli_version() -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(GET_LATEST_SPIN_CLI_URL).send()?;
    let latest_version = response.text()?;
    Ok(latest_version)
}
