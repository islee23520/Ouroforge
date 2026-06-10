//! Ouroforge protocol/data-model contracts.
//!
//! This crate keeps new protocol schemas out of the large `ouroforge-core`
//! compilation unit. Modules are explicit `pub mod` declarations only; no glob
//! re-exports are used.

pub mod behavior_authoring_spec;
