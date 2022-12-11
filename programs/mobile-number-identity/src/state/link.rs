use anchor_lang::prelude::*;

/// Link account serves as protection against duplicate identities.
/// If a link between the ID and the owner's public key is missing, the identity will be considered void.
#[account]
pub struct Link {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// Identity which this link is related to. (32)
    pub identity: Pubkey,
}

impl Link {
    pub fn len() -> usize {
        8 + 1 + 32
    }
}
