//! Spectra-backed self-telemetry for [Photon]: typed event/metric schemas, Photon topic
//! helpers, and an [`OpsLog`](photon_telemetry::OpsLog) adapter that forwards Photon's own
//! runtime signals (publishes, drains, backlog, handler failures) into [Spectra].
//!
//! [Photon]'s [`OpsLog`](photon_telemetry::OpsLog) trait is deliberately backend-agnostic:
//! Photon calls `record_counter` / `record_gauge` / `log_event` on whatever implementation the
//! host installs. This crate is that implementation for hosts that already emit their own
//! telemetry through [Spectra]: [`SpectraOpsLog`] forwards each call into `spectra-core`'s
//! non-recursive gate, and [`install_ops_log_from_env`] wires it up (or opts out) based on
//! `PHOTON_TELEMETRY`. Photon storage is a pluggable `StoragePort` (mem/sqlite/nats/…); this
//! crate does not lock hosts to a particular backend.
//!
//! [Photon]: https://github.com/unified-field-dev/photon
//! [Spectra]: https://github.com/unified-field-dev/spectra
//!
//! ## Features
//!
//! - **`OpsLog` install** — [`SpectraOpsLog`] implements [`photon_telemetry::OpsLog`] by
//!   routing counters, gauges, and events through `spectra-core`.
//! - **Env-driven install** — [`install_ops_log_from_env`] reads `PHOTON_TELEMETRY`
//!   (`off` / `console` / default-to-Spectra) and installs the matching log process-wide.
//! - **Typed schemas** — Spectra DSL schemas for Photon's own publish/drain/backlog counters
//!   and legacy-named `StoragePort` append/op wall-clock gauges (`continuum_*`), registered via
//!   `inventory` when linked.
//! - **Topic + codegen helpers** — generated `*Payload` / `*_TOPIC` DTOs and `*Recorder` /
//!   `*Logger` types, importable straight from the crate root (e.g.
//!   [`PhotonPublishesRecorder`]).
//!

//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---|---|
//! | Install process-wide `OpsLog` | [`install_ops_log_from_env`] / [`SpectraOpsLog`] |
//! | Typed counters / gauges / events | [`helpers`] (`*Recorder`, `*Logger`) |
//! | Transport DTOs + topic constants | [`topics`] (`*Payload`, `*_TOPIC`) |
//!
//! Labels for Photon's counters/gauges are supplied by callers via the `OpsLog::record_counter`
//! / `record_gauge` label slices; this crate has no dedicated label types.
//!
//! ## Generated schemas & topics
//!
//! Typed `*Recorder` / `*Logger` / `*Payload` / `*_TOPIC` symbols are re-exported at the crate
//! root and grouped under [`helpers`] and [`topics`]. One mid-level pattern for both surfaces:
//!
//! ```rust,no_run
//! use photon_spectra_telemetry::{
//!     PhotonPublishesPayload, PhotonPublishesRecorder, PHOTON_PUBLISHES_TOPIC,
//! };
//!
//! PhotonPublishesRecorder::record(1, serde_json::json!({"topic": "demo", "mode": "local"}));
//! assert_eq!(PhotonPublishesPayload::topic(), PHOTON_PUBLISHES_TOPIC);
//! ```
//!
//! See [`helpers`] for the full recorder/logger set and [`topics`] for transport DTOs.
//!
//! ## Environment
//!
//! | Variable | Values | Default |
//! |----------|--------|---------|
//! | `PHOTON_TELEMETRY` | `off`, `console`, `spectra` | `spectra` (when Spectra is configured) |
//!
//! # Getting started
//!
//! Install the Spectra sink first, then `OpsLog` install, before building your Photon runtime:
//!
//! ```rust,no_run
//! use photon_spectra_telemetry::install_ops_log_from_env;
//!
//! // Reads `PHOTON_TELEMETRY` (off / console / default-to-Spectra) and installs the
//! // matching `OpsLog` process-wide.
//! install_ops_log_from_env();
//!
//! // ... build and run your Photon host; Photon's own publish/drain/backlog events now
//! // flow through Spectra automatically.
//! ```
//!
//! Runnable: `cargo run -p photon-spectra-telemetry --example ops_log_smoke`.
//!
//! ## Where to look next
//!
//! - [`install_ops_log_from_env`] / [`SpectraOpsLog`] — process-wide `OpsLog` bootstrap
//! - [`helpers`] / [`topics`] — generated recorders, loggers, payloads, and topic constants

/// Typed emit helpers from Photon Spectra schemas.
pub mod helpers;
mod install;
mod metrics;
mod sanitize;
// macro-generated Spectra schema types; documented via each schema's `description`
#[allow(missing_docs)]
mod schemas;
/// Transport `*Payload` / `*_TOPIC` DTOs from Photon Spectra schemas.
pub mod topics;

pub use helpers::*;
pub use topics::*;

pub use install::{install_ops_log_from_env, SpectraOpsLog};
