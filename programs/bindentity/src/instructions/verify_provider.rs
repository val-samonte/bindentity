use anchor_lang::prelude::*;

use crate::state::{Identity, Link, Provider, Validator};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct VerifyProviderParams {
    pub owner_id: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(params: VerifyProviderParams)]
pub struct VerifyProvider<'info> {
    #[account(
        seeds = [
            "provider".as_bytes(),
            "provider".as_bytes(),
        ],
        bump = verifier_provider.bump
    )]
    pub verifier_provider: Account<'info, Provider>,

    #[account(
        constraint = validator.provider.key() == verifier_provider.key(),
    )]
    pub validator: Account<'info, Validator>,

    #[account(
        constraint = signer.key() == validator.signer.key(),
    )]
    pub signer: Signer<'info>,

    #[account(
        has_one = owner,
        seeds = [
            "identity".as_bytes(),
            owner_identity.timestamp.to_string().as_bytes(),
            verifier_provider.key().as_ref(),
            params.owner_id.as_ref(),
        ],
        bump = owner_identity.bump
    )]
    pub owner_identity: Account<'info, Identity>,

    #[account(
        constraint = owner_link.identity.key() == owner_identity.key(),
    )]
    pub owner_link: Account<'info, Link>,

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
