use anchor_lang::prelude::*;

use crate::{program::MobileNumberIdentity, state::Global};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeParams {
    pub validator: Pubkey,
    pub treasury: Pubkey,
    pub service_fee: u64,
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
        space = 8 + 1 + 32 + 32 + 32 + 8,
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, MobileNumberIdentity>,

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
    global.validator = params.validator.key();
    global.treasury = params.treasury.key();
    global.service_fee = params.service_fee;

    Ok(())
}
