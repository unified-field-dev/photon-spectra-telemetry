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
//! - **Env-resolved telemetry install** — Reads `PHOTON_TELEMETRY` at host boot and installs the matching
//!   process-wide `OpsLog` before the Photon runtime starts.
//!   [Get started](#env-driven-install)
//! - **Spectra `OpsLog` adapter** — [`SpectraOpsLog`] implements [`photon_telemetry::OpsLog`] when you wire
//!   the Spectra adapter yourself instead of using the env helper.
//!   [Get started](#direct-ops-log)
//! - **Topic + codegen helpers** — Generated `*Recorder` / `*Logger` / `*Payload` / `*_TOPIC`
//!   symbols for explicit Photon telemetry emits from host or test code.
//!   [Get started](#typed-recorders)
//! - **Typed schemas** — Spectra DSL schemas for Photon's publish/drain/backlog counters and
//!   legacy-named `StoragePort` append/op wall-clock gauges (`continuum_*`), registered via
//!   `inventory` when linked into a host.
//!
//! # Getting started
//!
//! Most hosts install the ops log once at startup, then let Photon's runtime emit through Spectra
//! automatically. Pick the env helper for production hosts or wire [`SpectraOpsLog`] directly when
//! tests need a fixed backend.
//!
//! ## Env-driven install
//!
//! [`install_ops_log_from_env`] is the default host path: it resolves `PHOTON_TELEMETRY` once at
//! process boot and registers the matching `OpsLog` before you build the Photon runtime, so
//! publish, drain, backlog, and handler-failure signals flow through Spectra for the process
//! lifetime.
//!
//! Prerequisites: Spectra must already be booted in the host process when `PHOTON_TELEMETRY` is
//! unset or set to `spectra`. Set `off` or `console` to disable or print locally.
//!
//! ```rust,no_run
//! // Call before constructing the Photon runtime.
//! photon_spectra_telemetry::install_ops_log_from_env();
//! let telemetry = std::env::var("PHOTON_TELEMETRY").unwrap_or_else(|_| "spectra".into());
//! assert!(!telemetry.trim().is_empty());
//! ```
//!
//! Runnable: `cargo run -p photon-spectra-telemetry --example ops_log_smoke`.
//!
//! Next: [Direct ops log](#direct-ops-log) when you need explicit wiring in tests.
//!
//! ## Direct ops log
//!
//! [`SpectraOpsLog`] is for hosts or tests that install `OpsLog` without reading
//! `PHOTON_TELEMETRY`. Construct the adapter and pass it to [`photon_telemetry::install_ops_log`]
//! before Photon starts emitting counters and events.
//!
//! Prerequisites: Spectra booted when using the default Spectra backend. Labels for counters and
//! gauges come from Photon callers via `OpsLog::record_counter` / `record_gauge` label slices.
//!
//! ```rust,no_run
//! use std::sync::Arc;
//!
//! use photon_spectra_telemetry::SpectraOpsLog;
//! use photon_telemetry::{install_ops_log, OpsLog};
//!
//! let log = SpectraOpsLog::new();
//! install_ops_log(Arc::new(log));
//! log.record_counter(
//!     "photon_publishes",
//!     &[("topic", "demo"), ("mode", "local")],
//!     1.0,
//! );
//! let metric = "photon_publishes";
//! assert_eq!(metric, "photon_publishes");
//! ```
//!
//! Next: [Typed recorders](#typed-recorders) when you emit Photon telemetry directly without
//! relying on the runtime `OpsLog` path.
//!
//! ## Typed recorders
//!
//! Generated `*Recorder` and `*Logger` types under [`helpers`] emit Photon counters and events
//! with typed labels and topic constants from [`topics`]. Call them from host code or tests when
//! you need an explicit emit instead of relying on Photon's runtime `OpsLog` path.
//!
//! Prerequisites: Spectra booted in the process. Import recorders from the crate root or
//! [`helpers`]; transport DTOs and `*_TOPIC` constants live in [`topics`].
//!
//! ```rust,no_run
//! use photon_spectra_telemetry::{
//!     PhotonPublishesPayload, PhotonPublishesRecorder, PHOTON_PUBLISHES_TOPIC,
//! };
//!
//! PhotonPublishesRecorder::record(
//!     1,
//!     serde_json::json!({"topic": "demo", "mode": "local"}),
//! );
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
//! # Feature flags
//!
//! This crate has no Cargo feature flags.

#![allow(clippy::too_long_first_doc_paragraph)]

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
