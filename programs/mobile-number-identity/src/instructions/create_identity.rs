use anchor_lang::{prelude::*, solana_program::hash::hashv, system_program};

use crate::state::{Global, Identity, Link};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateIdentityParams {
    id: String,
    timestamp: u32,
}

#[derive(Accounts)]
#[instruction(params: CreateIdentityParams)]
pub struct CreateIdentity<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [
            "identity".as_bytes(),
            params.timestamp.to_string().as_bytes(),
            params.id.as_bytes(),
        ],
        bump,
        space = 8 + 1 + 32 + 32 + 4,
    )]
    pub identity: Account<'info, Identity>,

    #[account(
        init,
        payer = owner,
        seeds = [
            "link".as_bytes(),
            params.id.as_bytes(),
            owner.key().as_ref(),
        ],
        bump,
        space = 8 + 1 + 32
    )]
    pub link: Account<'info, Link>,

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
    identity.timestamp = params.timestamp;
    identity.id = hashv(&[params.id.as_bytes().as_ref()])
        .to_bytes()
        .try_into()
        .unwrap();

    Ok(())
}
