use anchor_lang::prelude::*;
use anchor_spl::token::ID;

pub mod contexts;
pub mod instructions;
pub mod types;

use contexts::*;
use instructions::*;
use types::AuthorityType;

#[program]
pub mod hoard_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        initialize::handler(ctx, total_supply)
    }

    pub fn mint(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        mint::handler(ctx, amount)
    }

    pub fn burn(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        burn::handler(ctx, amount)
    }

    pub fn transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        transfer::handler(ctx, amount)
    }

    pub fn initialize_account(ctx: Context<InitializeAccountContext>) -> Result<()> {
        account::initialize_account(ctx)
    }

    pub fn initialize_mint(ctx: Context<InitializeMintContext>, decimals: u8) -> Result<()> {
        account::initialize_mint(ctx, decimals)
    }

    pub fn approve(ctx: Context<ApproveContext>, amount: u64) -> Result<()> {
        account::approve(ctx, amount)
    }

    pub fn revoke(ctx: Context<RevokeContext>) -> Result<()> {
        account::revoke(ctx)
    }

    pub fn set_authority(ctx: Context<SetAuthorityContext>, authority_type: AuthorityType, new_authority: Option<Pubkey>) -> Result<()> {
        account::set_authority(ctx, authority_type, new_authority)
    }

    pub fn close_account(ctx: Context<CloseAccountContext>) -> Result<()> {
        account::close_account(ctx)
    }
}
