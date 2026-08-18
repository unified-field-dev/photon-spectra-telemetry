//! [`OpsLog`] implementation routing Photon self-metrics to Spectra.

use std::sync::Arc;

use photon_telemetry::{install_ops_log, ConsoleOpsLog, NoOpsLog, OpsLog};

use crate::metrics;

/// Emits Photon ops metrics/events via `spectra-core` (non-recursive gate).
///
/// # Examples
///
/// ```rust,no_run
/// use photon_spectra_telemetry::SpectraOpsLog;
/// use photon_telemetry::OpsLog;
///
/// let log = SpectraOpsLog::new();
/// log.record_counter(
///     "photon_publishes",
///     &[("topic", "demo"), ("mode", "local")],
///     1.0,
/// );
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct SpectraOpsLog;

impl SpectraOpsLog {
    /// New Spectra-backed ops log.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use photon_spectra_telemetry::SpectraOpsLog;
    ///
    /// let _log = SpectraOpsLog::new();
    /// ```
    pub const fn new() -> Self {
        Self
    }
}

impl OpsLog for SpectraOpsLog {
    fn record_counter(&self, name: &str, labels: &[(&str, &str)], value: f64) {
        metrics::record_counter(name, labels, value);
    }

    fn record_gauge(&self, name: &str, labels: &[(&str, &str)], value: f64) {
        metrics::record_gauge(name, labels, value);
    }

    fn log_event(&self, name: &str, payload: &serde_json::Value) {
        metrics::log_event(name, payload);
    }
}

/// Install process-wide Photon ops log from `PHOTON_TELEMETRY`.
///
/// Contract:
///
/// - **When to call** — once per process, before building the Photon runtime (after Spectra's
///   own sink is configured when using default-to-Spectra mode).
/// - **Env values** — `off` / `0` / `false` / `none` → [`NoOpsLog`]; `console` →
///   [`ConsoleOpsLog`]; unset or `spectra` → [`SpectraOpsLog`] (may no-op when Spectra is
///   unconfigured).
/// - **Idempotency** — replaces the process-wide `OpsLog` on each call; hosts should not call
///   from hot paths.
///
/// # Examples
///
/// ```rust,no_run
/// use photon_spectra_telemetry::install_ops_log_from_env;
///
/// // Prefer setting `PHOTON_TELEMETRY` in the process environment before calling.
/// install_ops_log_from_env();
/// ```
///
/// See also `cargo run -p photon-spectra-telemetry --example ops_log_smoke`.
pub fn install_ops_log_from_env() {
    let log: Arc<dyn OpsLog> = match std::env::var("PHOTON_TELEMETRY")
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("off" | "0" | "false" | "none") => Arc::new(NoOpsLog),
        Some("console") => Arc::new(ConsoleOpsLog),
        _ => Arc::new(SpectraOpsLog::new()),
    };
    install_ops_log(log);
}
