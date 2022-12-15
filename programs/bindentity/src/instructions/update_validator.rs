use anchor_lang::prelude::*;

use crate::state::{Provider, Validator};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateValidatorParams {
    flags: u8,
}

#[derive(Accounts)]
#[instruction(params: UpdateValidatorParams)]
pub struct UpdateValidator<'info> {
    #[account(
        mut,
        constraint = validator.provider.key() == provider.key(),
    )]
    pub validator: Account<'info, Validator>,

    #[account(
        has_one = authority,
    )]
    pub provider: Account<'info, Provider>,

    pub authority: Signer<'info>,
}

pub fn update_validator_handler(
    ctx: Context<UpdateValidator>,
    params: UpdateValidatorParams,
) -> Result<()> {
    let validator = &mut ctx.accounts.validator;

    validator.flags = params.flags;

    Ok(())
}
