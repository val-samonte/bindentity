use anchor_lang::prelude::*;

/// A Provider is the owner of a specific ID name.
/// A Provider also manages several Validator accounts.
#[account]
pub struct Provider {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// Binary flags which describes the status of the provider. (1)
    ///
    /// * 1 - Enabled: System level control flag which tells if this Provider is enabled or not.
    /// * 2 - Published: Control flag which is being set by the Provider's Owner which tells whether if it is usable or not.
    /// * 4 - Verified: Owner of this Provider account is verified through mythrilsoft `provider` provider ID.
    pub flags: u8,

    /// Account that manages this identity provider. (32)
    pub authority: Pubkey,

    /// Account that holds the collected registration fees. (32)
    pub treasury: Pubkey,

    /// Default fee collected to the user when availing the offered service.
    /// If the validator is permitted to customize the fee, this field will be ignored. (8)
    pub registration_fee: u64,

    /// The unique name of the provider (eg. email, phone, metamask, ph_national_id). (Varies)
    pub name: String,

    /// URI which stores off chain details such as name, description, image, tags, etc. (Varies)
    pub uri: String,

    /// Unused reserved byte space for future additive changes. (32)
    pub _reserved: [u8; 32],
}

impl Provider {
    pub fn len(name: &String, uri: &String) -> usize {
        8 + 1 + 1 + 32 + 32 + 8 + (4 + name.len()) + (4 + uri.len()) + 32
    }
}
