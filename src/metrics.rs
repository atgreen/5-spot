// Prometheus metrics for 5-Spot controller
//
// This module provides observability metrics for monitoring the controller's
// health, performance, and operational state.

use std::sync::LazyLock;

use prometheus::{
    register_counter_vec, register_gauge, register_gauge_vec, register_histogram_vec, CounterVec,
    Gauge, GaugeVec, HistogramVec,
};

/// Total number of reconciliations performed
pub static RECONCILIATIONS_TOTAL: LazyLock<CounterVec> = LazyLock::new(|| {
    register_counter_vec!(
        "fivespot_reconciliations_total",
        "Total number of reconciliations performed",
        &["phase", "result"]
    )
    .expect("Failed to register reconciliations_total metric")
});

/// Duration of reconciliation operations in seconds
pub static RECONCILIATION_DURATION_SECONDS: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        "fivespot_reconciliation_duration_seconds",
        "Duration of reconciliation operations in seconds",
        &["phase"],
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
    )
    .expect("Failed to register reconciliation_duration_seconds metric")
});

/// Number of currently active machines (in Active phase)
pub static MACHINES_ACTIVE: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "fivespot_machines_active",
        "Number of machines currently in Active phase"
    )
    .expect("Failed to register machines_active metric")
});

/// Number of scheduled machines by phase
pub static MACHINES_BY_PHASE: LazyLock<GaugeVec> = LazyLock::new(|| {
    register_gauge_vec!(
        "fivespot_machines_by_phase",
        "Number of ScheduledMachine resources by phase",
        &["phase"]
    )
    .expect("Failed to register machines_by_phase metric")
});

/// Total number of schedule evaluations
pub static SCHEDULE_EVALUATIONS_TOTAL: LazyLock<CounterVec> = LazyLock::new(|| {
    register_counter_vec!(
        "fivespot_schedule_evaluations_total",
        "Total number of schedule evaluations",
        &["result"]
    )
    .expect("Failed to register schedule_evaluations_total metric")
});

/// Number of machines with kill switch activated
pub static KILL_SWITCH_ACTIVATIONS_TOTAL: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "fivespot_kill_switch_activations_total",
        "Total number of kill switch activations"
    )
    .expect("Failed to register kill_switch_activations_total metric")
});

/// Controller info gauge (always 1, used for labels)
pub static CONTROLLER_INFO: LazyLock<GaugeVec> = LazyLock::new(|| {
    register_gauge_vec!(
        "fivespot_controller_info",
        "Controller information",
        &["version", "instance_id"]
    )
    .expect("Failed to register controller_info metric")
});

/// Whether the controller is the leader (1 = leader, 0 = not leader)
pub static IS_LEADER: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(
        "fivespot_is_leader",
        "Whether this controller instance is the leader (1 = leader, 0 = not leader)"
    )
    .expect("Failed to register is_leader metric")
});

/// Number of errors by type
pub static ERRORS_TOTAL: LazyLock<CounterVec> = LazyLock::new(|| {
    register_counter_vec!(
        "fivespot_errors_total",
        "Total number of errors by type",
        &["error_type"]
    )
    .expect("Failed to register errors_total metric")
});

/// Node drain operations
pub static NODE_DRAINS_TOTAL: LazyLock<CounterVec> = LazyLock::new(|| {
    register_counter_vec!(
        "fivespot_node_drains_total",
        "Total number of node drain operations",
        &["result"]
    )
    .expect("Failed to register node_drains_total metric")
});

/// Pod evictions during node drain
pub static POD_EVICTIONS_TOTAL: LazyLock<CounterVec> = LazyLock::new(|| {
    register_counter_vec!(
        "fivespot_pod_evictions_total",
        "Total number of pod evictions during node drain",
        &["result"]
    )
    .expect("Failed to register pod_evictions_total metric")
});

/// Record a successful reconciliation
pub fn record_reconciliation_success(phase: &str, duration_secs: f64) {
    RECONCILIATIONS_TOTAL
        .with_label_values(&[phase, "success"])
        .inc();
    RECONCILIATION_DURATION_SECONDS
        .with_label_values(&[phase])
        .observe(duration_secs);
}

/// Record a failed reconciliation
pub fn record_reconciliation_failure(phase: &str, duration_secs: f64) {
    RECONCILIATIONS_TOTAL
        .with_label_values(&[phase, "failure"])
        .inc();
    RECONCILIATION_DURATION_SECONDS
        .with_label_values(&[phase])
        .observe(duration_secs);
}

/// Record a schedule evaluation result
pub fn record_schedule_evaluation(is_active: bool) {
    let result = if is_active { "active" } else { "inactive" };
    SCHEDULE_EVALUATIONS_TOTAL
        .with_label_values(&[result])
        .inc();
}

/// Update the count of machines in a specific phase
pub fn set_machines_by_phase(phase: &str, count: f64) {
    MACHINES_BY_PHASE.with_label_values(&[phase]).set(count);
}

/// Record an error by type
pub fn record_error(error_type: &str) {
    ERRORS_TOTAL.with_label_values(&[error_type]).inc();
}

/// Record a node drain result
pub fn record_node_drain(success: bool) {
    let result = if success { "success" } else { "failure" };
    NODE_DRAINS_TOTAL.with_label_values(&[result]).inc();
}

/// Record a pod eviction result
pub fn record_pod_eviction(success: bool) {
    let result = if success { "success" } else { "failure" };
    POD_EVICTIONS_TOTAL.with_label_values(&[result]).inc();
}

/// Initialize controller info metric
pub fn init_controller_info(version: &str, instance_id: u32) {
    CONTROLLER_INFO
        .with_label_values(&[version, &instance_id.to_string()])
        .set(1.0);
}

/// Set leader status
pub fn set_leader_status(is_leader: bool) {
    IS_LEADER.set(if is_leader { 1.0 } else { 0.0 });
}

#[cfg(test)]
#[path = "metrics_tests.rs"]
mod tests;
