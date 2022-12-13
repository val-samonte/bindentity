use anchor_lang::prelude::*;

use crate::state::Global;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateConfigParams {
    pub authority: Option<Pubkey>,
    pub treasury: Option<Pubkey>,
    pub service_fee: Option<u64>,
    pub provider_fee: Option<u64>,
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

    match params.service_fee {
        Some(service_fee) => {
            global.service_fee = service_fee;
        }
        None => (),
    }

    match params.provider_fee {
        Some(provider_fee) => {
            global.provider_fee = provider_fee;
        }
        None => (),
    }

    match params.treasury {
        Some(treasury) => {
            global.treasury = treasury.key();
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
