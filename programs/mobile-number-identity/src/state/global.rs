use anchor_lang::prelude::*;

#[account]
pub struct Global {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// The authority that is permitted to update this state. (32)
    pub authority: Pubkey,

    /// The wallet that stores the collected fees (used for external services payment). (32)
    pub treasury: Pubkey,

    /// Amount of fee being collected when a user registers his / her wallet. (8)
    pub service_fee: u64,

    /// Fee for registering an identity provider account. (8)
    pub provider_fee: u64,

    /// Unused reserved byte space for future additive changes. (128)
    pub _reserved: [u8; 128],
}

impl Global {
    pub fn len() -> usize {
        8 + 1 + 32 + 32 + 8 + 8 + 128
    }
}
