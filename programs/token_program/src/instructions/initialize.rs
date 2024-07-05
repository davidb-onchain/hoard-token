use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};

use crate::contexts::Initialize;

pub fn handler(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
    let seeds = &[ctx.accounts.mint.to_account_info().key.as_ref()];
    let signer = &[&seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.initial_token_account.to_account_info(),
        authority: ctx.accounts.mint.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    
    token::mint_to(cpi_context.with_signer(signer), total_supply)?;

    Ok(())
}
