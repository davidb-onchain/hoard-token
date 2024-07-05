use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{self, Token, Mint, MintTo, Burn, Transfer, InitializeAccount, InitializeMint, Approve, Revoke, SetAuthority, CloseAccount};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{Instruction, InstructionError},
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use borsh::{BorshSerialize, BorshDeserialize};

#[tokio::test]
async fn test_initialize() {
    // Setup test environment
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new(
        "hoard_token",
        program_id,
        processor!(hoard_token::entry)
    );
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    // Create test accounts
    let mint = Keypair::new();
    let authority = Keypair::new();
    let total_supply = 1_000_000_000;

    // Airdrop SOL to the payer
    let airdrop_signature = banks_client
        .request_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await
        .unwrap();
    banks_client
        .confirm_transaction(&airdrop_signature)
        .await
        .unwrap();

    // Initialize mint instruction
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(anchor_spl::token::ID, false),
        ],
        data: InitializeInstruction::new(total_supply).to_vec(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Verify mint initialization
    let mint_account = banks_client.get_account(mint.pubkey()).await.unwrap().unwrap();
    let mint_data = Mint::unpack(&mint_account.data).unwrap();
    assert_eq!(mint_data.supply, total_supply);
    assert_eq!(mint_data.decimals, 9); // Assuming 9 decimals
    assert_eq!(mint_data.mint_authority, COption::Some(authority.pubkey()));
}

#[tokio::test]
async fn test_mint() {
    // Setup test environment
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new(
        "hoard_token",
        program_id,
        processor!(hoard_token::entry)
    );
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    // Create test accounts
    let mint = Keypair::new();
    let token_account = Keypair::new();
    let authority = Keypair::new();
    let amount = 100;

    // Airdrop SOL to the payer
    let airdrop_signature = banks_client
        .request_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await
        .unwrap();
    banks_client
        .confirm_transaction(&airdrop_signature)
        .await
        .unwrap();

    // Mint tokens instruction
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(token_account.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(anchor_spl::token::ID, false),
        ],
        data: MintInstruction::new(amount).to_vec(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Verify minting
    let token_account_data = banks_client.get_account(token_account.pubkey()).await.unwrap().unwrap();
    let token_data = token::TokenAccount::unpack(&token_account_data.data).unwrap();
    assert_eq!(token_data.amount, amount);
}

#[tokio::test]
async fn test_burn() {
    // Setup test environment
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new(
        "hoard_token",
        program_id,
        processor!(hoard_token::entry)
    );
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    // Create test accounts
    let mint = Keypair::new();
    let token_account = Keypair::new();
    let authority = Keypair::new();
    let amount = 50;

    // Airdrop SOL to the payer
    let airdrop_signature = banks_client
        .request_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await
        .unwrap();
    banks_client
        .confirm_transaction(&airdrop_signature)
        .await
        .unwrap();

    // Burn tokens instruction
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(token_account.pubkey(), false),
            AccountMeta::new(mint.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(anchor_spl::token::ID, false),
        ],
        data: BurnInstruction::new(amount).to_vec(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &token_account],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Verify burning
    let token_account_data = banks_client.get_account(token_account.pubkey()).await.unwrap().unwrap();
    let token_data = token::TokenAccount::unpack(&token_account_data.data).unwrap();
    assert_eq!(token_data.amount, 0);
}

#[tokio::test]
async fn test_transfer() {
    // Setup test environment
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new(
        "hoard_token",
        program_id,
        processor!(hoard_token::entry)
    );
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    // Create test accounts
    let mint = Keypair::new();
    let from_token_account = Keypair::new();
    let to_token_account = Keypair::new();
    let authority = Keypair::new();
    let amount = 100;

    // Airdrop SOL to the payer
    let airdrop_signature = banks_client
        .request_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await
        .unwrap();
    banks_client
        .confirm_transaction(&airdrop_signature)
        .await
        .unwrap();

    // Transfer tokens instruction
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(from_token_account.pubkey(), false),
            AccountMeta::new(to_token_account.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(anchor_spl::token::ID, false),
        ],
        data: TransferInstruction::new(amount).to_vec(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &from_token_account],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Verify transfer
    let from_token_account_data = banks_client.get_account(from_token_account.pubkey()).await.unwrap().unwrap();
    let to_token_account_data = banks_client.get_account(to_token_account.pubkey()).await.unwrap().unwrap();
    let from_token_data = token::TokenAccount::unpack(&from_token_account_data.data).unwrap();
    let to_token_data = token::TokenAccount::unpack(&to_token_account_data.data).unwrap();
    assert_eq!(from_token_data.amount, 0);
    assert_eq!(to_token_data.amount, amount);
}

#[tokio::test]
async fn test_initialize_account() {
    // Setup test environment
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new(
        "hoard_token",
        program_id,
        processor!(hoard_token::entry)
    );
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    // Create test accounts
    let mint = Keypair::new();
    let token_account = Keypair::new();
    let authority = Keypair::new();

    // Airdrop SOL to the payer
    let airdrop_signature = banks_client
        .request_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await
        .unwrap();
    banks_client
        .confirm_transaction(&airdrop_signature)
        .await
        .unwrap();

    // Initialize account instruction
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(token_account.pubkey(), true),
            AccountMeta::new_readonly(mint.pubkey(), false),
            AccountMeta::new_readonly(authority.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(anchor_spl::token::ID, false),
        ],
        data: InitializeAccountInstruction::new().to_vec(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &token_account],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Verify account initialization
    let token_account_data = banks_client.get_account(token_account.pubkey()).await.unwrap().unwrap();
    let token_data = token::TokenAccount::unpack(&token_account_data.data).unwrap();
    assert_eq!(token_data.state, AccountState::Initialized);
}