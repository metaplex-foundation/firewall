use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};
use solana_program::program_memory::sol_memcmp;
use crate::error::AccountsError;

/// Checks Pubkeys for equality, uses a faster memcmp from solana
pub fn assert_key_equal(a: &Pubkey, b: &Pubkey) -> Result<(), AccountsError> {
    if sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) != 0 {
        return Err(AccountsError::KeyMismatch);
    }
    Ok(())
}

/// Checks key equality and returns a custom error
pub fn assert_key_equal_with_error(a: &Pubkey, b: &Pubkey, err: AccountsError) -> Result<(), AccountsError> {
    assert_key_equal(a, b).map_err(|_| err)
}

pub fn derive(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, program_id)
}

