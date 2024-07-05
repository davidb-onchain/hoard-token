use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};

use crate::contexts::MintTokens;

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    token::mint_to(ctx.accounts.into_mint_to_context(), amount)?;
    Ok(())
}

impl<'info> MintTokens<'info> {
    pub fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.to.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
