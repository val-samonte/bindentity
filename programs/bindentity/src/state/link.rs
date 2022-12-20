use anchor_lang::prelude::*;

/// Link account serves as protection against duplicate bindies.
/// If a link between the provider and the owner's data is missing, the bindie is considered void.
#[account]
pub struct Link {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// Bindie which this link is related to. (32)
    pub bindie: Pubkey,
}

impl Link {
    pub fn len() -> usize {
        8 + 1 + 32
    }
}
