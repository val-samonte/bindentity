use anchor_lang::prelude::*;

use crate::state::Provider;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateProviderParams {
    flags: Option<u8>,
    authority: Option<Pubkey>,
    treasury: Option<Pubkey>,
    registration_fee: Option<u64>,
    uri: Option<String>,
}

#[derive(Accounts)]
#[instruction(params: UpdateProviderParams)]
pub struct UpdateProvider<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub provider: Account<'info, Provider>,

    pub authority: Signer<'info>,
}

pub fn update_provider_handler(
    ctx: Context<UpdateProvider>,
    params: UpdateProviderParams,
) -> Result<()> {
    let provider = &mut ctx.accounts.provider;

    match params.flags {
        Some(flags) => {
            provider.flags = flags;
        }
        None => (),
    }

    match params.authority {
        Some(authority) => {
            provider.authority = authority;
        }
        None => (),
    }

    match params.treasury {
        Some(treasury) => {
            provider.treasury = treasury;
        }
        None => (),
    }

    match params.registration_fee {
        Some(registration_fee) => {
            provider.registration_fee = registration_fee;
        }
        None => (),
    }

    match params.uri {
        Some(uri) => {
            provider.uri = uri;
        }
        None => (),
    }

    Ok(())
}
