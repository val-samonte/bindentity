use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("3XNHaUkdcLNFKydAhtxhEtg2f9fmWi5Ggm4oUt8Yzy5N");

#[program]
pub mod bindentity {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        initialize_handler(ctx, params)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        update_config_handler(ctx, params)
    }

    pub fn create_provider(
        ctx: Context<CreateProvider>,
        params: CreateProviderParams,
    ) -> Result<()> {
        create_provider_handler(ctx, params)
    }

    pub fn create_provider_metadata(
        ctx: Context<CreateProviderMetadata>,
        params: CreateProviderMetadataParams,
    ) -> Result<()> {
        create_provider_metadata_handler(ctx, params)
    }

    pub fn create_validator(
        ctx: Context<CreateValidator>,
        params: CreateValidatorParams,
    ) -> Result<()> {
        create_validator_handler(ctx, params)
    }

    pub fn create_bindie(ctx: Context<CreateBindie>, params: CreateBindieParams) -> Result<()> {
        create_bindie_handler(ctx, params)
    }

    pub fn verify_provider(
        ctx: Context<VerifyProvider>,
        params: VerifyProviderParams,
    ) -> Result<()> {
        verify_provider_handler(ctx, params)
    }

    pub fn update_provider(
        ctx: Context<UpdateProvider>,
        params: UpdateProviderParams,
    ) -> Result<()> {
        update_provider_handler(ctx, params)
    }

    pub fn buy_provider(ctx: Context<BuyProvider>, params: BuyProviderParams) -> Result<()> {
        buy_provider_handler(ctx, params)
    }

    pub fn update_validator(
        ctx: Context<UpdateValidator>,
        params: UpdateValidatorParams,
    ) -> Result<()> {
        update_validator_handler(ctx, params)
    }

    pub fn void_bindie(ctx: Context<VoidBindie>, params: VoidBindieParams) -> Result<()> {
        void_bindie_handler(ctx, params)
    }
}

#[error_code]
pub enum CustomError {
    #[msg("Provider is disabled")]
    ProviderDisabled,

    #[msg("Provider is unpublished")]
    ProviderUnpublished,

    #[msg("Validator is not allowed to create a bindie")]
    ValidatorDisabled,

    #[msg("Hash of the data does not match")]
    InvalidDataHash,

    #[msg("Signers are not authorized to void")]
    VoidUnauthorized,

    #[msg("Cannot sell a disabled / published / provider with validators")]
    SellingNotAllowed,
}
