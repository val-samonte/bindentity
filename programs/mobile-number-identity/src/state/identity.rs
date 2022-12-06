use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    /// Bump nonce of the PDA (1).
    pub bump: u8,

    /// Owner of this identity (32)
    pub owner: Pubkey,
}
