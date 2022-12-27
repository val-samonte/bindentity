use anchor_lang::prelude::*;

use crate::state::{Provider, ProviderMetadata};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateProviderMetadataParams {
    uri: String,
}

#[derive(Accounts)]
#[instruction(params: CreateProviderMetadataParams)]
pub struct CreateProviderMetadata<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            "provider_metadata".as_bytes(),
            provider.key().as_ref(),
        ],
        bump,
        space = ProviderMetadata::len(&params.uri),
    )]
    pub provider_metadata: Account<'info, ProviderMetadata>,

    #[account(mut, has_one = authority)]
    pub provider: Account<'info, Provider>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_provider_metadata_handler(
    ctx: Context<CreateProviderMetadata>,
    params: CreateProviderMetadataParams,
) -> Result<()> {
    let metadata = &mut ctx.accounts.provider_metadata;
    let provider = &mut ctx.accounts.provider;

    metadata.bump = *ctx.bumps.get("provider_metadata").unwrap();
    metadata.uri = params.uri;
    provider.flags |= 32;

    Ok(())
}
