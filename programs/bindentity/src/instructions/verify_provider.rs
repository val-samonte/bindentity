use anchor_lang::prelude::*;

use crate::state::{Bindie, Link, Provider, Validator};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct VerifyProviderParams {
    pub data: String,
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
            "bindie".as_bytes(),
            owner_bindie.timestamp.to_string().as_bytes(),
            verifier_provider.key().as_ref(),
            Bindie::crop(&params.data).as_bytes(),
        ],
        bump = owner_bindie.bump
    )]
    pub owner_bindie: Account<'info, Bindie>,

    #[account(
        constraint = owner_link.bindie.key() == owner_bindie.key(),
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

    target_provider.flags |= 4;

    Ok(())
}
