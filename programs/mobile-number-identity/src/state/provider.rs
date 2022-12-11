use anchor_lang::prelude::*;

/// A Provider is the owner of a specific ID name.
/// A Provider also manages several Validator accounts.
#[account]
pub struct Provider {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// Account that manages this identity provider. (32)
    pub authority: Pubkey,

    /// Account that holds the collected registration fees. (32)
    pub treasury: Pubkey,

    /// Fee collected to the user when availing the offered service. (8)
    pub registration_fee: u64,

    /// The unique name of the provider (eg. email, phone, metamask, ph_national_id). (Varies)
    pub name: String,

    /// Unused reserved byte space for future additive changes. (32)
    pub _reserved: [u8; 32],
}

impl Provider {
    pub fn len(name: String) -> usize {
        8 + 1 + 32 + 32 + 8 + (4 + name.len()) + 32
    }
}
