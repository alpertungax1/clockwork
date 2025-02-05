//! Utility functions to make it easier to build programs with Clockwork.

use anchor_lang::{prelude::Pubkey, solana_program};
use static_pubkey::static_pubkey;

/// The stand-in pubkey for delegating a payer address to a worker. All workers are re-imbursed by the user for lamports spent during this delegation.
pub static PAYER_PUBKEY: Pubkey = static_pubkey!("C1ockworkPayer11111111111111111111111111111");

/// The sighash of a named instruction in an Anchor program.
pub fn anchor_sighash(name: &str) -> [u8; 8] {
    let namespace = "global";
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}
