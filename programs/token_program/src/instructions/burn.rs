use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn};

use crate::contexts::BurnTokens;

pub fn handler(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
    token::burn(ctx.accounts.into_burn_to_context(), amount)?;
    Ok(())
}

impl<'info> BurnTokens<'info> {
    pub fn into_burn_to_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.mint.to_account_info(),
            from: self.from.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
