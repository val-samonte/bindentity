use anchor_lang::{prelude::*, solana_program::hash::hashv, system_program};

use crate::{
    state::{Global, Identity, Link},
    CustomError,
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct VoidIdentityParams {
    id: Option<String>,
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

/// User can void an identity in 2 ways:
/// 1. If the validator already verified that the owner indeed owns the id
/// 2. If the identity account has the same owner field
pub fn void_identity_handler(ctx: Context<VoidIdentity>, params: VoidIdentityParams) -> Result<()> {
    let identity = &ctx.accounts.identity;
    let link = &mut ctx.accounts.link;
    let signer = &mut ctx.accounts.signer;

    match params.id {
        Some(id) => {
            let hash: [u8; 32] = hashv(&[id.as_bytes().as_ref()])
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
