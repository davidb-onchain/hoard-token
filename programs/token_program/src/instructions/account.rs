use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeAccount, InitializeMint, Approve, Revoke, SetAuthority, CloseAccount};
use crate::types::AuthorityType;

use crate::contexts::*;

pub fn initialize_account(ctx: Context<InitializeAccountContext>) -> Result<()> {
    let cpi_accounts = InitializeAccount {
        account: ctx.accounts.account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::initialize_account(cpi_context)?;
    Ok(())
}

pub fn initialize_mint(ctx: Context<InitializeMintContext>, decimals: u8) -> Result<()> {
    let cpi_accounts = InitializeMint {
        mint: ctx.accounts.mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::initialize_mint(cpi_context, decimals, ctx.accounts.mint_authority.key, None)?;
    Ok(())
}

pub fn approve(ctx: Context<ApproveContext>, amount: u64) -> Result<()> {
    let cpi_accounts = Approve {
        to: ctx.accounts.delegate.to_account_info(),
        delegate: ctx.accounts.delegate.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::approve(cpi_context, amount)?;
    Ok(())
}

pub fn revoke(ctx: Context<RevokeContext>) -> Result<()> {
    let cpi_accounts = Revoke {
        source: ctx.accounts.source.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::revoke(cpi_context)?;
    Ok(())
}

pub fn set_authority(ctx: Context<SetAuthorityContext>, authority_type: AuthorityType, new_authority: Option<Pubkey>) -> Result<()> {
    let cpi_accounts = SetAuthority {
        account_or_mint: ctx.accounts.account_or_mint.to_account_info(),
        current_authority: ctx.accounts.current_authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::set_authority(cpi_context, authority_type.into(), new_authority)?;
    Ok(())
}

pub fn close_account(ctx: Context<CloseAccountContext>) -> Result<()> {
    let cpi_accounts = CloseAccount {
        account: ctx.accounts.account.to_account_info(),
        destination: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::close_account(cpi_context)?;
    Ok(())
}

impl<'info> ApproveContext<'info> {
    pub fn into_approve_context(&self) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let cpi_accounts = Approve {
            to: self.delegate.to_account_info(),
            delegate: self.delegate.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> RevokeContext<'info> {
    pub fn into_revoke_context(&self) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let cpi_accounts = Revoke {
            source: self.source.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> SetAuthorityContext<'info> {
    pub fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.account_or_mint.to_account_info(),
            current_authority: self.current_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
