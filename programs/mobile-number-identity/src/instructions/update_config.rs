use anchor_lang::prelude::*;

use crate::state::Global;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateConfigParams {
    pub authority: Option<Pubkey>,
    pub validator: Option<Pubkey>,
    pub treasury: Option<Pubkey>,
    pub creation_fee: Option<u64>,
}

#[derive(Accounts)]
#[instruction(params: UpdateConfigParams)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = ["global".as_bytes()],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,

    #[account(
        mut,
        constraint = authority.key() == global.authority.key()
    )]
    pub authority: Signer<'info>,
}

pub fn update_config_handler(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
    let global = &mut ctx.accounts.global;

    match params.creation_fee {
        Some(creation_fee) => {
            global.creation_fee = creation_fee;
        }
        None => (),
    }

    match params.treasury {
        Some(treasury) => {
            global.treasury = treasury.key();
        }
        None => (),
    }

    match params.validator {
        Some(validator) => {
            global.validator = validator.key();
        }
        None => (),
    }

    match params.authority {
        Some(authority) => {
            global.authority = authority.key();
        }
        None => (),
    }

    Ok(())
}
