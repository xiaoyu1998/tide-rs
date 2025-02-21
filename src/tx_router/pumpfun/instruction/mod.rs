//! Instructions for interacting with the Pump.fun program.
//!
//! This module contains instruction builders for creating Solana instructions to interact with the
//! Pump.fun program. Each function takes the required accounts and instruction data and returns a
//! properly formatted Solana instruction.
//!
//! # Instructions
//!
//! - `create`: Instruction to create a new token with an associated bonding curve.
//! - `buy`: Instruction to buy tokens from a bonding curve by providing SOL.
//! - `sell`: Instruction to sell tokens back to the bonding curve in exchange for SOL.

// use borsh::{BorshSerialize, BorshDeserialize};
use crate::tx_router::pumpfun::{constants, PumpFun};

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct CreateInstructionArgs {
//     pub name: String,
//     pub symbol: String,
//     pub uri: String,
// }

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct BuyInstructionArgs {
//     pub amount: u64,
//     pub max_sol_cost: u64,
// }

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct SellInstructionArgs {
//     pub amount: u64,
//     pub min_sol_output: u64,
// }

use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};

use pumpfun_cpi as cpi;
use anchor_client::anchor_lang::InstructionData;

/// Creates an instruction to create a new token with bonding curve
///
/// Creates a new SPL token with an associated bonding curve that determines its price.
///
/// # Arguments
///
/// * `payer` - Keypair that will pay for account creation and transaction fees
/// * `mint` - Keypair for the new token mint account that will be created
/// * `args` - Create instruction data containing token name, symbol and metadata URI
///
/// # Returns
///
/// Returns a Solana instruction that when executed will create the token and its accounts
//pub fn create(payer: &Keypair, mint: &Keypair, args: CreateInstructionArgs) -> Instruction {
pub fn create(payer: &Keypair, mint: &Keypair, args: cpi::instruction::Create) -> Instruction {
    let bonding_curve: Pubkey = PumpFun::get_bonding_curve_pda(&mint.pubkey()).unwrap();

    // let mut args_bytes : Vec<u8> = Vec::new();
    // args.serialize(&mut args_bytes).unwrap(); 

    Instruction::new_with_bytes(
        constants::accounts::PUMPFUN,
        //&args_bytes,
        &args.data(),
        vec![
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(PumpFun::get_mint_authority_pda(), false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(
                get_associated_token_address(&bonding_curve, &mint.pubkey()),
                false,
            ),
            AccountMeta::new_readonly(PumpFun::get_global_pda(), false),
            AccountMeta::new_readonly(constants::accounts::MPL_TOKEN_METADATA, false),
            AccountMeta::new(PumpFun::get_metadata_pda(&mint.pubkey()), false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(constants::accounts::SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::TOKEN_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::ASSOCIATED_TOKEN_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::RENT, false),
            AccountMeta::new_readonly(constants::accounts::EVENT_AUTHORITY, false),
            AccountMeta::new_readonly(constants::accounts::PUMPFUN, false),
        ],
    )
}

/// Creates an instruction to buy tokens from a bonding curve
///
/// Buys tokens by providing SOL. The amount of tokens received is calculated based on
/// the bonding curve formula. A portion of the SOL is taken as a fee and sent to the
/// fee recipient account.
///
/// # Arguments
///
/// * `payer` - Keypair that will provide the SOL to buy tokens
/// * `mint` - Public key of the token mint to buy
/// * `fee_recipient` - Public key of the account that will receive the transaction fee
/// * `args` - Buy instruction data containing the SOL amount and maximum acceptable token price
///
/// # Returns
///
/// Returns a Solana instruction that when executed will buy tokens from the bonding curve
pub fn buy(
    payer: &Keypair,
    mint: &Pubkey,
    fee_recipient: &Pubkey,
    //args: BuyInstructionArgs,
    args: cpi::instruction::Buy,
) -> Instruction {
    let bonding_curve: Pubkey = PumpFun::get_bonding_curve_pda(mint).unwrap();
    // let mut args_bytes : Vec<u8> = Vec::new();
    // args.serialize(&mut args_bytes).unwrap(); 

    Instruction::new_with_bytes(
        constants::accounts::PUMPFUN,
        //&args_bytes,
        &args.data(),
        vec![
            AccountMeta::new_readonly(PumpFun::get_global_pda(), false),
            AccountMeta::new(*fee_recipient, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(get_associated_token_address(&bonding_curve, mint), false),
            AccountMeta::new(get_associated_token_address(&payer.pubkey(), mint), false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(constants::accounts::SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::TOKEN_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::RENT, false),
            AccountMeta::new_readonly(constants::accounts::EVENT_AUTHORITY, false),
            AccountMeta::new_readonly(constants::accounts::PUMPFUN, false),
        ],
    )
}

/// Creates an instruction to sell tokens back to a bonding curve
///
/// Sells tokens back to the bonding curve in exchange for SOL. The amount of SOL received
/// is calculated based on the bonding curve formula. A portion of the SOL is taken as
/// a fee and sent to the fee recipient account.
///
/// # Arguments
///
/// * `payer` - Keypair that owns the tokens to sell
/// * `mint` - Public key of the token mint to sell
/// * `fee_recipient` - Public key of the account that will receive the transaction fee
/// * `args` - Sell instruction data containing token amount and minimum acceptable SOL output
///
/// # Returns
///
/// Returns a Solana instruction that when executed will sell tokens to the bonding curve
pub fn sell(
    payer: &Keypair,
    mint: &Pubkey,
    fee_recipient: &Pubkey,
    //args: SellInstructionArgs,
    args: cpi::instruction::Sell,
) -> Instruction {
    let bonding_curve: Pubkey = PumpFun::get_bonding_curve_pda(mint).unwrap();
    // let mut args_bytes : Vec<u8> = Vec::new();
    // args.serialize(&mut args_bytes).unwrap(); 

    Instruction::new_with_bytes(
        constants::accounts::PUMPFUN,
        //&args_bytes,
        &args.data(),
        vec![
            AccountMeta::new_readonly(PumpFun::get_global_pda(), false),
            AccountMeta::new(*fee_recipient, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(get_associated_token_address(&bonding_curve, mint), false),
            AccountMeta::new(get_associated_token_address(&payer.pubkey(), mint), false),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(constants::accounts::SYSTEM_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::ASSOCIATED_TOKEN_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::TOKEN_PROGRAM, false),
            AccountMeta::new_readonly(constants::accounts::EVENT_AUTHORITY, false),
            AccountMeta::new_readonly(constants::accounts::PUMPFUN, false),
        ],
    )
}
