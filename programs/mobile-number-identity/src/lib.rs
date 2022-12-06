use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("FQvpwXKBzpt9QzMUwMnnKXe4PkE4XPq6dgujiPhCfjGE");

#[program]
pub mod mobile_number_identity {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        initialize_handler(ctx, params)
    }

    pub fn create_identity(
        ctx: Context<CreateIdentity>,
        params: CreateIdentityParams,
    ) -> Result<()> {
        create_identity_handler(ctx, params)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        update_config_handler(ctx, params)
    }
}
