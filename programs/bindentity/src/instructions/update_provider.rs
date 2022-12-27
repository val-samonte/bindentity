use anchor_lang::prelude::*;

use crate::{state::Provider, CustomError};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateProviderParams {
    for_sale: Option<bool>,
    published: Option<bool>,
    selling_price: Option<u64>,
    authority: Option<Pubkey>,
    treasury: Option<Pubkey>,
    registration_fee: Option<u64>,
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

    match params.for_sale {
        Some(for_sale) => {
            if for_sale {
                // provider should be enabled
                if provider.flags & 1 != 1 
                    // provider should be unpublished
                    || provider.flags & 2 == 2 
                    // provider should have no validators
                    || provider.validator_count != 0 
                {
                    return Err(error!(CustomError::SellingNotAllowed));
                }
                provider.flags |= 8;
            } else {
                // remove sell flag (8)
                provider.flags &= 65527; 
            }
        }
        None => (),
    }

    match params.selling_price {
        Some(selling_price) => {
            provider.selling_price = selling_price;
        }
        None => (),
    }

    if provider.flags & 8 == 8 {
        return Ok(());
    }
    
    match params.published {
        Some(published) => {
            // provider must have metadata before allowing to get published
            if published && provider.flags & 32 == 32 {
                provider.flags |= 2;
            } else {
                // remove published flag (2)
                provider.flags &= 65533
            }
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

    Ok(())
}
