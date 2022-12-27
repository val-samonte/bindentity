use anchor_lang::prelude::*;

use crate::state::{Provider, ProviderMetadata};

#[derive(Accounts)]
pub struct RemoveProviderMetadata<'info> {
    #[account(
        mut,
        seeds = [
            "provider_metadata".as_bytes(),
            provider.key().as_ref(),
        ],
        bump = provider_metadata.bump,
    )]
    pub provider_metadata: Account<'info, ProviderMetadata>,

    #[account(mut, has_one = authority)]
    pub provider: Account<'info, Provider>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn remove_provider_metadata_handler(ctx: Context<RemoveProviderMetadata>) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    let provider = &mut ctx.accounts.provider;
    let metadata = &mut ctx.accounts.provider_metadata;

    let source_account_info = metadata.to_account_info();
    let dest_account_info = authority.to_account_info();

    let dest_starting_lamports = dest_account_info.lamports();
    **dest_account_info.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(source_account_info.lamports())
        .unwrap();
    **source_account_info.lamports.borrow_mut() = 0;

    let mut source_data = source_account_info.data.borrow_mut();
    source_data.fill(0);

    // remove published flag (2) and `has metadata` flag (32)
    provider.flags &= 65501;

    Ok(())
}
