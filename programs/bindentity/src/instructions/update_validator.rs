use anchor_lang::prelude::*;

use crate::state::{Provider, Validator};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateValidatorParams {
    close: Option<bool>,
    flags: Option<u8>,
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
        mut,
        has_one = authority,
    )]
    pub provider: Account<'info, Provider>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn update_validator_handler(
    ctx: Context<UpdateValidator>,
    params: UpdateValidatorParams,
) -> Result<()> {
    let provider = &mut ctx.accounts.provider;
    let validator = &mut ctx.accounts.validator;

    match params.close {
        Some(close) => {
            if close {
                let source_account_info = validator.to_account_info();
                let dest_account_info = ctx.accounts.authority.to_account_info();

                let dest_starting_lamports = dest_account_info.lamports();
                **dest_account_info.lamports.borrow_mut() = dest_starting_lamports
                    .checked_add(source_account_info.lamports())
                    .unwrap();
                **source_account_info.lamports.borrow_mut() = 0;

                let mut source_data = source_account_info.data.borrow_mut();
                source_data.fill(0);

                provider.validator_count -= 1;

                if provider.validator_count == 0 {
                    // remove `has validator` flag
                    provider.flags &= 239;
                }

                return Ok(());
            }
        }
        None => match params.flags {
            Some(flags) => {
                validator.flags = flags;
            }
            None => (),
        },
    }

    Ok(())
}
