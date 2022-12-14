use anchor_lang::prelude::*;

/// A Validator is a delegate of the Provider which holds the authorized Signer.
/// A Signer is authorized to verify and approve any identity-related transaction.
/// It can be a person, a multisig, or a keypair securely stored in the Provider's backend server.
#[account]
pub struct Validator {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// Binary flags which describes the status of the validator. (1)
    ///
    /// * 1 - Enabled: Control flag set by the parent Provider on whether this Validator is allowed to operate or not.
    /// * 2 - Permitted to customize `registration_fee`.
    pub flags: u8,

    /// Authority who owns this validator. (32)
    pub provider: Pubkey,

    /// Account that is permitted to verify and approve transactions. (32)
    pub signer: Pubkey,
}

impl Validator {
    pub fn len() -> usize {
        8 + 1 + 1 + 32 + 32
    }
}
