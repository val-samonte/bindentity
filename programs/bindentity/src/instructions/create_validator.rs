use anchor_lang::prelude::*;

use crate::state::{Provider, Validator};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateValidatorParams {
    enabled: bool,
    signer: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: CreateValidatorParams)]
pub struct CreateValidator<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            "validator".as_bytes(),
            provider.key().as_ref(),
            params.signer.key().as_ref(),
        ],
        bump,
        space = Validator::len(),
    )]
    pub validator: Account<'info, Validator>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        constraint = provider.flags & 8 != 8,
    )]
    pub provider: Account<'info, Provider>,

    pub system_program: Program<'info, System>,
}

pub fn create_validator_handler(
    ctx: Context<CreateValidator>,
    params: CreateValidatorParams,
) -> Result<()> {
    let provider = &mut ctx.accounts.provider;
    let validator = &mut ctx.accounts.validator;

    validator.bump = *ctx.bumps.get("validator").unwrap();
    validator.flags = if params.enabled { 1 } else { 0 };
    validator.signer = params.signer.key();
    validator.provider = provider.key();
    provider.validator_count += 1;

    Ok(())
}
