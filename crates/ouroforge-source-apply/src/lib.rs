//! Source-apply governance modules.
//!
//! Extracted from `ouroforge-core` to shrink the core crate's compilation unit.
//! Each module is self-contained (depends only on `serde`/`serde_json`/`anyhow`,
//! with no `crate::`/`super::` references). `ouroforge-core` re-exports every
//! module so existing `ouroforge_core::source_apply_*` paths keep working.

pub mod source_apply_audit_ledger;
pub mod source_apply_emergency_hold;
pub mod source_apply_evidence_bundle;
pub mod source_apply_highrisk_blocker;
pub mod source_apply_post_apply_rerun;
pub mod source_apply_review_enforcement;
pub mod source_apply_rollback_snapshot;
pub mod source_apply_sandbox_promotion;
pub mod source_apply_verification_runner;
