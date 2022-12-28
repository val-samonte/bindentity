use anchor_lang::{prelude::*, solana_program::hash::hashv};

/// A Bindie is an account that proves the association of the given data to its owner's wallet account.
/// A Bindie is considered void if the corresponding Link account is closed.
#[account]
pub struct Bindie {
    /// Bump nonce of the PDA. (1)
    pub bump: u8,

    /// Owner of this identity. (32)
    pub owner: Pubkey,

    /// The authority who issued this identity. (32)
    pub provider: Pubkey,

    /// Unix timestamp. Besides from acting as a seed, useful to get the identities' creation order. (4)
    pub timestamp: u64,

    /// How many times the data has been encrypted with SHA 256.
    /// If the value is non-zero, program will automatically encrypt the data once with additional seed (provider name) before storing
    /// * 0 - Not encrypted (not recommended, data stored in the blockchain as is)
    /// * 1 - Encrypted once (data is exposed when passed via rpc request)
    /// * 2 - Encrypted twice (recommended, data is encrypted during validation before being sent to the rpc)
    pub encryption_count: u8,

    /// Any data, such as hashed phone number or hashed email. (32 / Varies)
    pub data: String,
}

impl Bindie {
    pub fn len(data: String) -> usize {
        8 + 1 + 32 + 32 + 8 + 1 + (4 + data.len())
    }

    pub fn data_hash(provider_name: &String, data: &String) -> String {
        hashv(&[provider_name.as_bytes(), ":".as_bytes(), data.as_ref()]).to_string()
    }

    pub fn crop(data: &String) -> String {
        if data.len() > 32 {
            data[..32].to_string()
        } else {
            data.clone()
        }
    }
}
