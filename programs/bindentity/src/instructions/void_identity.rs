use anchor_lang::{prelude::*, solana_program::hash::hashv, system_program};

use crate::{
    state::{Global, Identity, Link, Provider, Validator},
    CustomError,
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct VoidIdentityParams {
    id: Option<Vec<u8>>,
}

#[derive(Accounts)]
#[instruction(params: VoidIdentityParams)]
pub struct VoidIdentity<'info> {
    pub identity: Account<'info, Identity>,

    #[account(
        mut,
        constraint = link.identity.key() == identity.key(),
    )]
    pub link: Account<'info, Link>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        constraint = validator_signer.key() == validator.signer.key(),
    )]
    pub validator_signer: Signer<'info>,

    #[account(
        constraint = validator.provider.key() == provider.key(),
        constraint = validator.flags & 1 == 1, // @ CustomError::ValidatorDisabled
    )]
    pub validator: Box<Account<'info, Validator>>,

    // #[account(
    //     constraint = provider.flags & 1 == 1, // @ CustomError::ProviderDisabled
    // )]
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
    pub global: Account<'info, Global>,

    pub system_program: Program<'info, System>,
}

/// An identity can be void in 2 ways:
/// 1. If the owner of the identity is also the signer
/// 2. If the validator checked that indeed the user is the owner of the ID
pub fn void_identity_handler(ctx: Context<VoidIdentity>, params: VoidIdentityParams) -> Result<()> {
    let identity = &ctx.accounts.identity;
    let provider = &ctx.accounts.provider;
    let link = &mut ctx.accounts.link;
    let signer = &mut ctx.accounts.signer;

    match params.id {
        Some(id) => {
            let hash: [u8; 32] = hashv(&[provider.name.as_bytes(), ":".as_bytes(), id.as_ref()])
                .to_bytes()
                .try_into()
                .unwrap();

            if hash != identity.id {
                return Err(error!(CustomError::InvalidIdHash));
            }
        }
        None => {
            if identity.key() != signer.key() {
                return Err(error!(CustomError::VoidUnauthorized));
            }
        }
    }

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: signer.to_account_info(),
            to: ctx.accounts.treasury.to_account_info(),
        },
    );

    system_program::transfer(cpi_ctx, ctx.accounts.global.service_fee)?;

    let source_account_info = link.to_account_info();
    let dest_account_info = signer.to_account_info();

    let dest_starting_lamports = dest_account_info.lamports();
    **dest_account_info.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(source_account_info.lamports())
        .unwrap();
    **source_account_info.lamports.borrow_mut() = 0;

    let mut source_data = source_account_info.data.borrow_mut();
    source_data.fill(0);

    Ok(())
}
