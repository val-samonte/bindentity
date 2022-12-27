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
    /// * 2 - Published: Control flag set by the Provider's Owner which tells whether if it is usable or not.
    /// * 4 - Verified: Owner of this Provider account is verified through protocol's `provider` provider bindie.
    /// * 8 - Listed for sale: `validator_count` must be 0 and the provider should be unpublished.
    /// * 16 - Has validator: for filter purposes, true if `validator_count` is greater than 0.
    /// * 32 - Has metadata
    pub flags: u16,

    /// Account that manages this identity provider. (32)
    pub authority: Pubkey,

    /// Account that holds the collected registration fees. (32)
    pub treasury: Pubkey,

    /// Default fee collected to the user when availing the offered service.
    /// If the validator is permitted to customize the fee, this field will be ignored. (8)
    pub registration_fee: u64,

    /// Number of validators under this provider. (4)
    pub validator_count: u32,

    /// The selling price when put up for sale, in lamports. (8)
    /// Note that the `validator_count` must be 0 in order for this provider to be listed.
    pub selling_price: u64,

    /// Unused reserved byte space for future additive changes. (32)
    pub _reserved: [u8; 32],

    /// The unique name of the provider (eg. email, phone, metamask, ph_national_id). (Varies)
    pub name: String,
}

impl Provider {
    pub fn len(name: &String) -> usize {
        8 + 1 + 2 + 32 + 32 + 8 + 4 + 8 + 32 + (4 + name.len())
    }
}
