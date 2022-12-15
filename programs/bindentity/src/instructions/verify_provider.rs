use anchor_lang::prelude::*;

use crate::state::{Identity, Provider};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct VerifyProviderParams {
    pub owner_id: String,
}

#[derive(Accounts)]
#[instruction(params: VerifyProviderParams)]
pub struct VerifyProvider<'info> {
    #[account(
        seeds = [
            "provider".as_bytes(),
            "provider".as_bytes(),
        ],
        bump = provider_provider.bump
    )]
    pub provider_provider: Account<'info, Provider>,

    #[account(
        has_one = owner,
        seeds = [
            "identity".as_bytes(),
            owner_identity.timestamp.to_string().as_bytes(),
            provider_provider.key().as_ref(),
            params.owner_id.as_bytes(),
        ],
        bump = owner_identity.bump
    )]
    pub owner_identity: Account<'info, Identity>,

    #[account(
        mut,
        constraint = target_provider.authority.key() == owner.key(),
    )]
    pub target_provider: Account<'info, Provider>,

    pub owner: Signer<'info>,
}

pub fn verify_provider_handler(
    ctx: Context<VerifyProvider>,
    _params: VerifyProviderParams,
) -> Result<()> {
    let target_provider = &mut ctx.accounts.target_provider;

    target_provider.flags = target_provider.flags | 4;

    Ok(())
}
