//! GDD (Game Design Document) planning modules.
//!
//! Extracted from `ouroforge-core` to shrink the core crate's compilation unit
//! and let these self-contained planning modules compile in parallel. Each
//! module is independent (depends only on `serde`/`serde_json`/`anyhow`).
//! `ouroforge-core` re-exports every module so existing
//! `ouroforge_core::gdd_*` paths keep working unchanged.

pub mod gdd_asset_placeholder_plan;
pub mod gdd_design_brief;
pub mod gdd_feasibility_gate;
pub mod gdd_gameplay_behavior_plan;
pub mod gdd_mechanics_mapping;
pub mod gdd_project_scaffold_plan;
pub mod gdd_prototype_apply;
pub mod gdd_prototype_draft_bundle;
pub mod gdd_prototype_evidence;
pub mod gdd_prototype_evidence_bundle;
pub mod gdd_prototype_task_graph;
pub mod gdd_requirement_extraction;
pub mod gdd_scenario_acceptance_plan;
pub mod gdd_scene_level_plan;
