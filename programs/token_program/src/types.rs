// programs/token_program/src/types.rs
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::AuthorityType as SplAuthorityType;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
}

impl From<AuthorityType> for SplAuthorityType {
    fn from(authority_type: AuthorityType) -> Self {
        match authority_type {
            AuthorityType::MintTokens => SplAuthorityType::MintTokens,
            AuthorityType::FreezeAccount => SplAuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => SplAuthorityType::AccountOwner,
            AuthorityType::CloseAccount => SplAuthorityType::CloseAccount,
        }
    }
}
