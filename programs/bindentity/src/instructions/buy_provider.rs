use anchor_lang::{prelude::*, system_program};

use crate::{state::Provider, CustomError};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BuyProviderParams {
    provider_treasury: Pubkey,
    registration_fee: u64,
    uri: String,
}

#[derive(Accounts)]
#[instruction(params: BuyProviderParams)]
pub struct BuyProvider<'info> {
    #[account(
        mut,
        constraint = (provider.flags & 8 == 8) @ CustomError::SellingNotAllowed,
        constraint = provider.authority.key() == seller.key()
    )]
    provider: Account<'info, Provider>,

    #[account(mut)]
    /// CHECK: constraint to the provider's authority
    seller: UncheckedAccount<'info>,

    #[account(mut)]
    buyer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn buy_provider_handler(ctx: Context<BuyProvider>, params: BuyProviderParams) -> Result<()> {
    let buyer = &mut ctx.accounts.buyer;
    let seller = &mut ctx.accounts.seller;
    let provider = &mut ctx.accounts.provider;

    // transfer funds from buyer to seller
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: buyer.to_account_info(),
            to: seller.to_account_info(),
        },
    );

    system_program::transfer(cpi_ctx, provider.selling_price)?;

    // reset provider as if newly created, and assign new authority
    provider.authority = buyer.key();
    provider.flags = 1;
    provider.treasury = params.provider_treasury.key();
    provider.registration_fee = params.registration_fee;
    provider.validator_count = 0;
    provider.selling_price = 0;
    provider.uri = params.uri;

    Ok(())
}
