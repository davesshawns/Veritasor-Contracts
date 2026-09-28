//! Attestation contract dynamic fees.
//!
//! This module provides dynamic fee calculation functions that adjust based
//! on the number of businesses involved in an attestation. Fees scale
//! proportionally with business count, enabling flexible pricing without
//! on-chain governance changes for each adjustment.

pub mod dynamic_fees;

pub use dynamic_fees::{
    calculate_dynamic_fee, get_business_count, get_total_business_count,
};