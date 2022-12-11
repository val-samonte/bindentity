use anchor_lang::prelude::*;

/// A Validator holds the account (owner) of who is permitted to verify an identity-related transaction.
/// A Validator Owner can be a person, a multisig, or a keypair securely stored in the provider's server.
#[account]
pub struct Validator {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// Binary flags which describes the validator. (1)
    pub flags: u8,

    /// Authority who manages this validator. (32)
    pub provider: Pubkey,

    /// Account that holds this validator. (32)
    pub owner: Pubkey,
}

impl Validator {
    pub fn len() -> usize {
        8 + 1 + 1 + 32 + 32
    }
}
