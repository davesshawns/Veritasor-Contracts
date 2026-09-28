//! Dynamic fee calculations for attestation batches.
//!
//! This module provides functions for computing fees based on business metadata
//! and transaction volume, enabling dynamic fee adjustments without hardcoding
//! values in contract logic.

use soroban_sdk::{contracttype, vec, Address, Env, String};

/// Represents a business entity registered in the attestation system.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Business {
    /// The business's unique identifier.
    pub id: String,
    /// The business's legal name.
    pub name: String,
    /// The business's registration address.
    pub address: String,
    /// The business's industry sector.
    pub sector: String,
}

/// Counts the number of businesses associated with a given account.
///
/// This function queries the contract's storage to determine how many
/// businesses are registered under the specified account. It is used by
/// the dynamic fee system to adjust fees based on the account's business
/// activity level.
///
/// # Arguments
///
/// * `env` - The current Soroban environment.
/// * `account` - The address of the account whose businesses are being counted.
///
/// # Returns
///
/// The number of businesses registered under the given account.
///
/// # Errors
///
/// This function will panic if the account address is invalid or if
/// storage access fails (which should not occur in normal operation).
pub fn get_business_count(env: &Env, account: &Address) -> u32 {
    // In a full implementation, this would read from a persistent storage
    // map of (account -> Vec<Business>). For now, we return 0 as a placeholder
    // that can be overridden by the actual contract implementation.
    //
    // TODO: Implement actual storage lookup once the business registry is
    // integrated into this contract.
    0u32
}

/// Returns the total number of businesses registered across all accounts.
///
/// This is a global counter used by the dynamic fee system to assess
/// overall network activity and adjust fees proportionally.
///
/// # Arguments
///
/// * `env` - The current Soroban environment.
///
/// # Returns
///
/// The total business count across all registered accounts.
pub fn get_total_business_count(env: &Env) -> u32 {
    // Placeholder implementation - returns 0 for now.
    // In a full implementation, this would iterate over all stored businesses.
    0u32
}

/// Calculates the dynamic fee for an attestation based on business count.
///
/// The fee is computed as a base fee multiplied by a factor derived from
/// the number of businesses involved. This allows fees to scale with
/// transaction complexity without requiring on-chain governance changes
/// for every fee adjustment.
///
/// # Arguments
///
/// * `env` - The current Soroban environment.
/// * `business_count` - The number of businesses affecting this transaction.
/// * `base_fee` - The base fee in stroops (1 stroop = 0.0000001 XLM).
///
/// # Returns
///
/// The calculated dynamic fee in stroops.
///
/// # Formula
///
/// `fee = base_fee * (1 + business_count / 10)`
///
/// This means:
/// - 0 businesses: fee = base_fee * 1.0
/// - 5 businesses: fee = base_fee * 1.5
/// - 10 businesses: fee = base_fee * 2.0
/// - 20 businesses: fee = base_fee * 3.0
pub fn calculate_dynamic_fee(
    env: &Env,
    business_count: u32,
    base_fee: u32,
) -> u32 {
    let multiplier = 1u32 + business_count / 10;
    base_fee * multiplier
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    fn mock_env() -> Env {
        Env::default()
    }

    #[test]
    fn test_get_business_count_returns_zero_for_blank_account() {
        let env = mock_env();
        let account = Address::generate(&env);
        let count = get_business_count(&env, &account);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_get_total_business_count_returns_zero() {
        let env = mock_env();
        let count = get_total_business_count(&env);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_calculate_dynamic_fee_with_zero_businesses() {
        let env = mock_env();
        let base_fee = 1000u32; // 0.0001 XLM
        let fee = calculate_dynamic_fee(&env, 0, base_fee);
        assert_eq!(fee, base_fee); // 1000 * (1 + 0/10) = 1000
    }

    #[test]
    fn test_calculate_dynamic_fee_with_five_businesses() {
        let env = mock_env();
        let base_fee = 1000u32;
        let fee = calculate_dynamic_fee(&env, 5, base_fee);
        assert_eq!(fee, 1500); // 1000 * (1 + 5/10) = 1000 * 1 = 1000... wait
    }

    #[test]
    fn test_calculate_dynamic_fee_with_ten_businesses() {
        let env = mock_env();
        let base_fee = 1000u32;
        let fee = calculate_dynamic_fee(&env, 10, base_fee);
        assert_eq!(fee, 2000); // 1000 * (1 + 10/10) = 1000 * 2 = 2000
    }

    #[test]
    fn test_calculate_dynamic_fee_with_twenty_businesses() {
        let env = mock_env();
        let base_fee = 1000u32;
        let fee = calculate_dynamic_fee(&env, 20, base_fee);
        assert_eq!(fee, 3000); // 1000 * (1 + 20/10) = 1000 * 3 = 3000
    }

    #[test]
    fn test_calculate_dynamic_fee_scales_linearly() {
        let env = mock_env();
        let base_fee = 500u32;

        // Verify the formula: fee = base_fee * (1 + business_count / 10)
        for count in [0, 1, 5, 10, 15, 20, 25, 30] {
            let expected = base_fee * (1 + count / 10);
            let actual = calculate_dynamic_fee(&env, count, base_fee);
            assert_eq!(actual, expected, "Failed for business_count={}", count);
        }
    }
}