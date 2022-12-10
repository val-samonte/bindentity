use anchor_lang::prelude::*;

#[account]
pub struct Global {
    /// Bump nonce of the PDA (1).
    pub bump: u8,

    /// The authority that is permitted to update this state (32)
    pub authority: Pubkey,

    /// The authority that validates the association of the phone number to the user's wallet inside Firebase Functions (32)
    pub validator: Pubkey,

    /// The wallet that stores the collected fees (used for Firebase services payments) (32)
    pub treasury: Pubkey,

    /// Amount of fee being collected when a user registers his / her wallet (8)
    pub service_fee: u64,
}
