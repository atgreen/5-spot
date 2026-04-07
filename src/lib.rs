// Library exports for 5Spot Machine Scheduler

pub mod constants;
pub mod crd;
pub mod health;
pub mod labels;
pub mod metrics;
pub mod reconcilers;

// Re-export main types
pub use crd::ScheduledMachine;
pub use health::HealthState;
pub use metrics::{
    init_controller_info, record_error, record_node_drain, record_pod_eviction,
    record_reconciliation_failure, record_reconciliation_success, record_schedule_evaluation,
    set_leader_status, set_machines_by_phase,
};
pub use reconcilers::{Context, ReconcilerError};
