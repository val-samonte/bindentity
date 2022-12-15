use anchor_lang::prelude::*;

use crate::{program::Bindentity, state::Global};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeParams {
    pub treasury: Pubkey,
    pub service_fee: u64,
    pub provider_creation_fee: u64,
}

#[derive(Accounts)]
#[instruction(params: InitializeParams)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            "global".as_bytes(),
        ],
        bump,
        space = Global::len(),
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, Bindentity>,

    #[account(
      constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub program_data: Box<Account<'info, ProgramData>>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let global = &mut ctx.accounts.global;

    global.bump = *ctx.bumps.get("global").unwrap();
    global.authority = ctx.accounts.authority.key();
    global.treasury = params.treasury.key();
    global.provider_creation_fee = params.provider_creation_fee;
    global.service_fee = params.service_fee;

    Ok(())
}
