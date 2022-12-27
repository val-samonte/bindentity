use anchor_lang::prelude::*;

/// A Provider Metadata stores the URI of the off-chain json file for additional information about the Provider
#[account]
pub struct ProviderMetadata {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// URI which stores off chain details in JSON: (Varies)
    ///
    /// * `name: string` (the user friendly name of the id)
    /// * `description: string`
    /// * `image: string`
    /// * `tags: string[]` (for filter purposes, common tags are: country code, utility, type of ID)
    /// * `website: string` (url of the marketing website)
    /// * `registration_url: string` (important! url to redirect the user when availing the provider's service)
    pub uri: String,
}

impl ProviderMetadata {
    pub fn len(uri: &String) -> usize {
        8 + 1 + (4 + uri.len())
    }
}
