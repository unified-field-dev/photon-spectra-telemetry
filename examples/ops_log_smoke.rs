//! Install Photon `OpsLog` from env and emit one typed counter.
//!
//! ```bash
//! PHOTON_TELEMETRY=console CARGO_BUILD_JOBS=1 \
//!   cargo run -p photon-spectra-telemetry --example ops_log_smoke
//! ```
//!
//! Success: `ops_log_smoke: OK`.

#![allow(clippy::print_stdout)]

use photon_spectra_telemetry::{install_ops_log_from_env, PhotonPublishesRecorder};

fn main() {
    std::env::set_var("PHOTON_TELEMETRY", "console");
    install_ops_log_from_env();
    PhotonPublishesRecorder::record(1, serde_json::json!({"topic": "demo", "mode": "local"}));
    println!("ops_log_smoke: OK");
}
