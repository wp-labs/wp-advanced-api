//! Enrichment API for Warp Flow: define the minimal traits for field enrichment.
//!
//! This crate exposes stable interfaces for implementing enrichment logic
//! outside the core `wp_engine` crate, similar to `wp-source-api` and
//! `wp-sink-api`.

use wp_model_core::model::DataField;

/// Enrichment capability: mutate `fields` based on a target anchor and needs list.
/// Return true if enrichment has been applied (or attempted) successfully.
pub trait EnrichingAble: Send + Sync {
    fn enriching(&self, fields: &mut Vec<DataField>, target: &DataField, needs: &[String]) -> bool;
}

/// Enrichment registry lookup interface.
pub trait EnrichLibAble: Send + Sync {
    fn get(&self, key: &str) -> Option<&dyn EnrichingAble>;
}

// Friendly aliases (optional ergonomic names)
pub type Enricher = dyn EnrichingAble;
pub type EnrichRegistry = dyn EnrichLibAble;
