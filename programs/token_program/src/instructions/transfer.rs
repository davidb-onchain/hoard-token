use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

use crate::contexts::TransferTokens;

pub fn handler(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    let seeds = &[ctx.accounts.from.to_account_info().key.as_ref()];
    let signer = &[&seeds[..]];

    let fee = amount * 4 / 100; // 4% fee
    let amount_with_fee = amount - fee;

    // Calculate 2% fee for ecosystem treasury and 2% for reward treasury
    let ecosystem_fee = fee / 2;
    let reward_fee = fee - ecosystem_fee;

    token::transfer(ctx.accounts.into_transfer_to_context().with_signer(signer), amount_with_fee)?;
    token::transfer(ctx.accounts.into_transfer_fee_context().with_signer(signer), ecosystem_fee)?;
    token::transfer(ctx.accounts.into_transfer_reward_context().with_signer(signer), reward_fee)?;
    Ok(())
}

impl<'info> TransferTokens<'info> {
    pub fn into_transfer_to_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.from.to_account_info(),
            to: self.to.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn into_transfer_fee_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.from.to_account_info(),
            to: self.ecosystem_treasury.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn into_transfer_reward_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.from.to_account_info(),
            to: self.reward_treasury.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
