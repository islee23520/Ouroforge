//! Ouroforge protocol/data-model contracts.
//!
//! This crate keeps new protocol schemas out of the large `ouroforge-core`
//! compilation unit. Modules are explicit `pub mod` declarations only; no glob
//! re-exports are used.

pub mod before_after_comparison;
pub mod behavior_authoring_spec;
pub mod behavior_parameter_preview;
pub mod behavior_scenario_assertions;
pub mod proposal_workbench_model;
