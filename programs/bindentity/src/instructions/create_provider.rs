use anchor_lang::{prelude::*, system_program};

use crate::state::{Global, Provider};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateProviderParams {
    name: String,
    published: bool,
    registration_fee: u64,
    uri: String,
    provider_treasury: Pubkey,
}

#[derive(Accounts)]
#[instruction(params: CreateProviderParams)]
pub struct CreateProvider<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [
            "provider".as_bytes(),
            params.name.as_bytes(),
        ],
        bump,
        space = Provider::len(&params.name, &params.uri),
    )]
    pub provider: Account<'info, Provider>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = treasury.key() == global.treasury.key()
    )]
    /// CHECK: constraint to global treasury
    pub treasury: UncheckedAccount<'info>,

    #[account(
        seeds = ["global".as_bytes()],
        bump = global.bump
    )]
    pub global: Box<Account<'info, Global>>,

    pub system_program: Program<'info, System>,
}

pub fn create_provider_handler(
    ctx: Context<CreateProvider>,
    params: CreateProviderParams,
) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    let provider = &mut ctx.accounts.provider;

    // pay provider creation fee
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: owner.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        },
    );

    system_program::transfer(cpi_ctx, ctx.accounts.global.provider_creation_fee)?;

    provider.bump = *ctx.bumps.get("provider").unwrap();
    provider.authority = owner.key();
    provider.flags = 1 | (if params.published { 2 } else { 0 });
    provider.treasury = params.provider_treasury.key();
    provider.registration_fee = params.registration_fee;
    provider.name = params.name;
    provider.uri = params.uri;

    Ok(())
}
