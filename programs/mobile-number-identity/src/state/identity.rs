use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    /// Bump nonce of the PDA (1).
    pub bump: u8,

    /// The hash of the phone number (32)
    pub id: [u8; 32],

    /// String in bytes of the series (YYYYDDD, eg. 2022340) (7)
    pub series: [u8; 7],

    /// Owner of this identity (32)
    pub owner: Pubkey,
}
