use crate::config::Config;
use erdp::ErrorDisplay;
use std::process::ExitCode;

mod config;

fn main() -> ExitCode {
    // Let the user know something is working.
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    println!("Starting {name} {version}.");

    // Load configurations.
    match Config::load() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to load configurations: {}.", e.display());
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
