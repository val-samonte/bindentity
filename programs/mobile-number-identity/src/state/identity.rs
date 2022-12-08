use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    /// Bump nonce of the PDA (1).
    pub bump: u8,

    /// The hash of the phone number. (32)
    pub id: [u8; 32],

    /// Owner of this identity. (32)
    pub owner: Pubkey,

    /// Unix timestamp. Besides from acting as a seed, useful to get the latest identity. (4)
    pub timestamp: u32,

    /// Wallet address that voided this identity. If present, this identity is considered void. (1 + 32)
    pub voided_by: Option<Pubkey>,
}
