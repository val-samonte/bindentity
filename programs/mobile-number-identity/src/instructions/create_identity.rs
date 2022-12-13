use anchor_lang::{prelude::*, solana_program::hash::hashv, system_program};

use crate::state::{Global, Identity, Link, Provider, Validator};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateIdentityParams {
    id: String,
    timestamp: u32,
    registration_fee: Option<u64>,
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
            provider.key().as_ref(),
            params.id.as_bytes(),
        ],
        bump,
        space = Identity::len(),
    )]
    pub identity: Box<Account<'info, Identity>>,

    #[account(
        init,
        payer = owner,
        seeds = [
            "link".as_bytes(),
            provider.key().as_ref(),
            params.id.as_bytes(),
        ],
        bump,
        space = Link::len(),
    )]
    pub link: Box<Account<'info, Link>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        constraint = signer.key() == validator.signer.key(),
    )]
    pub signer: Signer<'info>,

    #[account(
        constraint = validator.provider.key() == provider.key(),
        constraint = validator.flags & 1 == 1, // @ CustomError::ValidatorDisabled
    )]
    pub validator: Box<Account<'info, Validator>>,

    #[account(
        mut,
        constraint = provider_treasury.key() == provider.treasury.key()
    )]
    /// CHECK: constraint to provider's treasury
    pub provider_treasury: UncheckedAccount<'info>,

    #[account(
        constraint = provider.flags & 1 == 1, // @ CustomError::ProviderDisabled
        constraint = provider.flags & 2 == 2, // @ CustomError::ProviderUnpublished
    )]
    pub provider: Box<Account<'info, Provider>>,

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

pub fn create_identity_handler(
    ctx: Context<CreateIdentity>,
    params: CreateIdentityParams,
) -> Result<()> {
    let identity = &mut ctx.accounts.identity;
    let link = &mut ctx.accounts.link;
    let owner = &mut ctx.accounts.owner;
    let provider = &ctx.accounts.provider;
    let validator = &ctx.accounts.validator;

    // pay service fee
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: owner.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        },
    );

    system_program::transfer(cpi_ctx, ctx.accounts.global.service_fee)?;

    // pay registration fee
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: owner.to_account_info(),
            to: ctx.accounts.provider_treasury.to_account_info(),
        },
    );

    system_program::transfer(
        cpi_ctx,
        if validator.flags & 2 == 2 && params.registration_fee.is_some() {
            params.registration_fee.unwrap()
        } else {
            provider.registration_fee
        },
    )?;

    link.bump = *ctx.bumps.get("link").unwrap();
    link.identity = identity.key();

    identity.bump = *ctx.bumps.get("identity").unwrap();
    identity.owner = owner.key();
    identity.provider = ctx.accounts.provider.key();
    identity.timestamp = params.timestamp;
    identity.id = hashv(&[params.id.as_bytes().as_ref()])
        .to_bytes()
        .try_into()
        .unwrap();

    Ok(())
}
