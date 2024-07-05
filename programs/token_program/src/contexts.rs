use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount, Mint};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, mint::decimals = 9, mint::authority = authority)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub initial_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub ecosystem_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reward_treasury: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InitializeAccountContext<'info> {
    #[account(mut)]
    pub account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InitializeMintContext<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub mint_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ApproveContext<'info> {
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    pub delegate: Signer<'info>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RevokeContext<'info> {
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetAuthorityContext<'info> {
    #[account(mut)] 
    /// CHECK: This account can be either a token account or a mint account. It is checked at runtime
    /// to ensure it is of the expected type.
    pub account_or_mint: AccountInfo<'info>,
    pub current_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is a system program account which does not require further checks.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseAccountContext<'info> {
    #[account(mut)]
    pub account: Account<'info, TokenAccount>,
    /// CHECK: This account is safe to use because it is a destination account where the remaining SOL will be transferred.
    pub destination: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
