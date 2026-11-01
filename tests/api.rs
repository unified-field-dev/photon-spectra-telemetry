//! Happy/sad coverage for install, `SpectraOpsLog`, typed recorders, and topics.
#![allow(missing_docs)]

use std::sync::Mutex;

use chrono::Utc;
use photon_spectra_telemetry::{
    install_ops_log_from_env, ContinuumAppendWallMsRecorder, ContinuumOpWallMsRecorder,
    PhotonBacklogRecorder, PhotonDlqLogLogger, PhotonDrainsRecorder, PhotonHandlerFailuresRecorder,
    PhotonOpsLogLogger, PhotonPublishErrorsRecorder, PhotonPublishesRecorder, SpectraOpsLog,
    CONTINUUM_APPEND_WALL_MS_TOPIC, CONTINUUM_OP_WALL_MS_TOPIC, PHOTON_BACKLOG_TOPIC,
    PHOTON_DLQ_LOG_TOPIC, PHOTON_DRAINS_TOPIC, PHOTON_HANDLER_FAILURES_TOPIC, PHOTON_OPS_LOG_TOPIC,
    PHOTON_PUBLISHES_TOPIC, PHOTON_PUBLISH_ERRORS_TOPIC,
};
use photon_telemetry::OpsLog;
use serde_json::json;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn install_ops_log_off_aliases_and_spectra_default_happy() {
    let _guard = ENV_LOCK.lock().expect("env lock");

    for off in ["off", "0", "false", "none"] {
        std::env::set_var("PHOTON_TELEMETRY", off);
        install_ops_log_from_env();
    }

    std::env::set_var("PHOTON_TELEMETRY", "console");
    install_ops_log_from_env();

    std::env::set_var("PHOTON_TELEMETRY", "spectra");
    install_ops_log_from_env();

    std::env::remove_var("PHOTON_TELEMETRY");
    install_ops_log_from_env();
}

#[test]
fn spectra_ops_log_counter_gauge_event_happy() {
    let log = SpectraOpsLog::new();
    log.record_counter("photon_publishes", &[("topic", "t")], 1.0);
    // fractional counters truncate toward zero via the adapter
    log.record_counter("photon_publishes", &[("topic", "t")], 2.9);
    log.record_gauge("photon_backlog", &[("topic", "t")], 3.0);
    log.log_event(
        "photon_ops_log",
        &json!({"component": "runtime", "message": "ready"}),
    );
}

#[test]
fn spectra_ops_log_unknown_and_empty_fields_accepted_sad() {
    let log = SpectraOpsLog::new();
    // unknown names / empty labels / empty payload are dropped — must not panic
    log.record_counter("unknown_photon_metric", &[], 0.0);
    log.record_gauge("unknown_photon_gauge", &[], -1.0);
    log.log_event("unknown_photon_event", &json!({}));
    log.record_counter("photon_publish_errors", &[], 0.0);
    log.log_event("photon_dlq", &json!({}));
}

#[test]
fn typed_recorders_emit_without_spectra_sink_happy() {
    let ts = Utc::now();
    PhotonPublishesRecorder::record_at(1, json!({"topic": "demo"}), ts);
    PhotonDrainsRecorder::record_at(1, json!({"topic": "demo"}), ts);
    PhotonBacklogRecorder::record_at(3, json!({"topic": "demo"}), ts);
    PhotonPublishErrorsRecorder::record_at(1, json!({"topic": "demo"}), ts);
    PhotonHandlerFailuresRecorder::record_at(1, json!({"topic": "demo"}), ts);
    ContinuumAppendWallMsRecorder::record_at(12, json!({"op": "append"}), ts);
    ContinuumOpWallMsRecorder::record_at(8, json!({"op": "read"}), ts);
    PhotonOpsLogLogger::log_at(
        "runtime".into(),
        "boot".into(),
        "ready".into(),
        "demo".into(),
        String::new(),
        String::new(),
        ts,
    );
    PhotonDlqLogLogger::log_at(
        "evt-1".into(),
        "demo".into(),
        "k".into(),
        1,
        "sub".into(),
        "handler_error".into(),
        "boom".into(),
        ts,
    );
}

#[test]
fn typed_recorders_empty_labels_accepted_sad() {
    let ts = Utc::now();
    PhotonPublishesRecorder::record_at(0, json!({}), ts);
    ContinuumAppendWallMsRecorder::record_at(0, json!({}), ts);
    PhotonOpsLogLogger::log_at(
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        ts,
    );
}

#[test]
fn topic_constants_are_non_empty_happy() {
    for topic in [
        PHOTON_PUBLISHES_TOPIC,
        PHOTON_DRAINS_TOPIC,
        PHOTON_BACKLOG_TOPIC,
        PHOTON_PUBLISH_ERRORS_TOPIC,
        PHOTON_HANDLER_FAILURES_TOPIC,
        CONTINUUM_APPEND_WALL_MS_TOPIC,
        CONTINUUM_OP_WALL_MS_TOPIC,
    ] {
        assert_ne!(topic, "");
        assert!(
            topic.starts_with("spectra.metric."),
            "unexpected metric topic: {topic}"
        );
        assert!(
            topic.contains("photon_") || topic.contains("continuum_"),
            "unexpected metric topic stem: {topic}"
        );
    }
    for topic in [PHOTON_OPS_LOG_TOPIC, PHOTON_DLQ_LOG_TOPIC] {
        assert_ne!(topic, "");
        assert!(
            topic.starts_with("spectra.event."),
            "unexpected event topic: {topic}"
        );
        assert!(
            topic.contains("photon_"),
            "unexpected event topic stem: {topic}"
        );
    }
}
