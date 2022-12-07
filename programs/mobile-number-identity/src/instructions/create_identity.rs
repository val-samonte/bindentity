use anchor_lang::{prelude::*, solana_program::hash::hashv, system_program};

use crate::state::{Global, Identity};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateIdentityParams {
    phone_number: String,
    series: String,
}

#[derive(Accounts)]
#[instruction(params: CreateIdentityParams)]
pub struct CreateIdentity<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [
            "identity".as_bytes(),
            params.series.as_bytes(),
            params.phone_number.as_bytes(),
        ],
        bump,
        space = 8 + 1 + 32 + 7 + 32,
    )]
    pub identity: Account<'info, Identity>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        constraint = validator.key() == global.validator.key()
    )]
    pub validator: Signer<'info>,

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
    pub global: Account<'info, Global>,

    pub system_program: Program<'info, System>,
}

pub fn create_identity_handler(
    ctx: Context<CreateIdentity>,
    params: CreateIdentityParams,
) -> Result<()> {
    let identity = &mut ctx.accounts.identity;
    let owner = &mut ctx.accounts.owner;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: owner.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        },
    );

    system_program::transfer(cpi_ctx, ctx.accounts.global.creation_fee)?;

    identity.bump = *ctx.bumps.get("identity").unwrap();
    identity.owner = owner.key();
    identity.series = params.series.as_bytes()[..7].try_into().unwrap();
    identity.id = hashv(&[params.phone_number.into_bytes().as_ref()]).to_bytes()[..32]
        .try_into()
        .unwrap();

    Ok(())
}