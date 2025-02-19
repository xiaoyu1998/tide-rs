pub mod accounts;
pub mod constants;
pub mod error;
pub mod instruction;
pub mod utils;

use borsh::BorshDeserialize;
//use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Signer, Keypair, Signature},
    compute_budget::ComputeBudgetInstruction,
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    transaction::Transaction,
};

use solana_program::{
    hash::Hash,
};

use anchor_spl::associated_token::{
    get_associated_token_address,
    spl_associated_token_account::instruction::create_associated_token_account,
};
// //declare_id!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
// fn hex_to_pubkey(mint_str: &String) -> Result<Pubkey, String> {
//     // Convert the string to a 32-byte array
//     let mint_bytes = match hex::decode(mint_str.to_string()) {
//         Ok(bytes) if bytes.len() == 32 => bytes,
//         Ok(_) => return Err("Invalid pubkey length".to_string()),
//         Err(err) => return Err(format!("Failed to decode pubkey string: {}", err)),
//     };

//     // Create Pubkey from the 32-byte array
//     let pubkey = Pubkey::new_from_array(mint_bytes.try_into().unwrap());
//     Ok(pubkey)
// }
//const PROGRAM_ID: Pubkey = hex_to_pubkey(&"6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string()).expect("Invalid program ID string");
//pub const PROGRAM_ID: Pubkey = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x6E, 0xF8, 0x72, 0x65, 0x74, 0x63, 0x68, 0x52,
    0x5D, 0x6B, 0x7A, 0x6F, 0x6E, 0x8E, 0xB8, 0x87,
    0x72, 0xB4, 0x12, 0x93, 0x80, 0x61, 0xE7, 0xF9,
    0x1D, 0x27, 0x14, 0x65, 0xB5, 0xD3, 0xD1, 0x73,
]);

/// Configuration for priority fee compute unit parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriorityFee {
    /// Maximum compute units that can be consumed by the transaction
    pub limit: Option<u32>,
    /// Price in micro-lamports per compute unit
    pub price: Option<u64>,
}

/// Main client for interacting with the Pump.fun program
pub struct PumpFun<'a> {
    /// RPC client for Solana network requests
    pub rpc: RpcClient,
    /// Keypair used to sign transactions
    pub payer: &'a Keypair,

    //pub program_id: Pubkey,
}

async fn fetch_recent_blockhash(rpc: &RpcClient) -> Result<Hash, solana_client::client_error::ClientError> {
    // Fetch the recent blockhash
    let latest_blockhash = rpc.get_latest_blockhash();
    //Ok(blockhash_response)
    return latest_blockhash;
}


impl<'a> PumpFun<'a> {
    /// Creates a new PumpFun client instance
    ///
    /// # Arguments
    ///
    /// * `payer` - Keypair used to sign and pay for transactions
    /// * `options` - Optional commitment config for transaction finality
    ///
    /// # Returns
    ///
    /// Returns a new PumpFun client instance configured with the provided parameters
    pub fn new(
        url: & String,
        payer: &'a Keypair,
        options: Option<CommitmentConfig>,
    ) -> Self {
        //Create Solana RPC Client with either WS or HTTP endpoint
        let rpc: RpcClient = RpcClient::new(url);

        // Return configured PumpFun client
        Self {
            rpc,
            payer,
            //program_id: hex_to_pubkey(&"6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string()).unwrap();
        }
    }

    /// Creates a new token with metadata by uploading metadata to IPFS and initializing on-chain accounts
    ///
    /// # Arguments
    ///
    /// * `mint` - Keypair for the new token mint account that will be created
    /// * `metadata` - Token metadata including name, symbol, description and image file
    /// * `priority_fee` - Optional priority fee configuration for compute units
    ///
    /// # Returns
    ///
    /// Returns the transaction signature if successful, or a ClientError if the operation fails
    // pub async fn create(
    //     &self,
    //     mint: &Keypair,
    //     metadata: utils::CreateTokenMetadata,
    //     priority_fee: Option<PriorityFee>,
    // ) -> Result<Signature, error::ClientError> {
    //     let recent_blockhash = fetch_recent_blockhash(self.rpc).await?;

    //     let mut instructions: Vec<Instruction> = Vec::new();

    //     // Add priority fee if provided
    //     if let Some(fee) = priority_fee {
    //         if let Some(limit) = fee.limit {
    //             let limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(limit);
    //             instructions.push(limit_ix);
    //         }

    //         if let Some(price) = fee.price {
    //             let price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
    //             instructions.push(price_ix);
    //         }
    //     }

    //     // Create the instruction for token creation
    //     let create_token_ix = instruction::create(payer, mint, instruction::CreateInstructionArgs {
    //         name: metadata.name,
    //         symbol: metadata.symbol,
    //         uri: metadata.file,
    //     });  
        
    //     instructions.push(create_token_ix);     

    //     // Build the transaction
    //     let mut transaction = Transaction::new_with_payer(&instructions, Some(&self.payer.pubkey()));

    //     // Sign the transaction with payer and mint
    //     let signers: Vec<&dyn Signer> = vec![&self.payer, mint];
    //     transaction.sign(&signers, recent_blockhash);

    //     // Send the transaction
    //     let signature = self.rpc.send_and_confirm_transaction(&transaction).await.map_err(|e| {
    //         error::ClientError::SolanaClientError(e.to_string())
    //     })?;


    //     Ok(signature)
    // }

    /// Creates a new token and immediately buys an initial amount in a single atomic transaction
    ///
    /// # Arguments
    ///
    /// * `mint` - Keypair for the new token mint
    /// * `metadata` - Token metadata to upload to IPFS
    /// * `amount_sol` - Amount of SOL to spend on initial buy in lamports
    /// * `slippage_basis_points` - Optional maximum acceptable slippage in basis points (1 bp = 0.01%). Defaults to 500
    /// * `priority_fee` - Optional priority fee configuration for compute units
    ///
    /// # Returns
    ///
    /// Returns the transaction signature if successful, or a ClientError if the operation fails
    pub async fn create_and_buy(
        &self,
        mint: &Keypair,
        metadata: utils::CreateTokenMetadata,
        amount_sol: u64,
        slippage_basis_points: Option<u64>,
        priority_fee: Option<PriorityFee>,
    ) -> Result<Signature, error::ClientError> {

        // Get accounts and calculate buy amounts
        let global_account = self.get_global_account()?;
        let buy_amount = global_account.get_initial_buy_price(amount_sol);
        let buy_amount_with_slippage =
            utils::calculate_with_slippage_buy(amount_sol, slippage_basis_points.unwrap_or(500));

        // Get the recent blockhash
        let recent_blockhash = match fetch_recent_blockhash(&self.rpc).await {
            Ok(blockhash) => blockhash,
            Err(e) => return  Err(error::ClientError::SolanaClientError(e)),
        };

        // Build the transaction instructions
        let mut instructions: Vec<Instruction> = Vec::new();

        // Add priority fee if provided
        if let Some(fee) = priority_fee {
            if let Some(limit) = fee.limit {
                let limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(limit);
                instructions.push(limit_ix);
            }

            if let Some(price) = fee.price {
                let price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
                instructions.push(price_ix);
            }
        }

        // Create the instruction for token creation
        let create_token_ix = instruction::create(self.payer, mint, instruction::CreateInstructionArgs {
            name: metadata.name,
            symbol: metadata.symbol,
            uri: metadata.file,
        });  
        
        instructions.push(create_token_ix); 

        // Create Associated Token Account if needed
        let ata: Pubkey = get_associated_token_address(&self.payer.pubkey(), &mint.pubkey());
        if self.rpc.get_account(&ata).is_err() {
            let ata_ix = create_associated_token_account(
                &self.payer.pubkey(),
                &self.payer.pubkey(),
                &mint.pubkey(),
                &constants::accounts::TOKEN_PROGRAM,
            );
            instructions.push(ata_ix);
        }

        // Add the buy instruction to the instructions vector
        let buy_args = instruction::BuyInstructionArgs {
            amount: buy_amount,
            max_sol_cost: buy_amount_with_slippage,
        };
        let buy_ix = instruction::buy(self.payer, &mint.pubkey(), &global_account.fee_recipient, buy_args);
        instructions.push(buy_ix);

        // Build the transaction
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&self.payer.pubkey()));

        // Sign the transaction
        let signers: Vec<&dyn solana_sdk::signer::Signer> = vec![&self.payer, mint];
        transaction.sign(&signers, recent_blockhash);

        // Send the transaction
        let signature = self.rpc.send_and_confirm_transaction(&transaction).map_err(|e| {
            error::ClientError::SolanaClientError(e)
        })?;

        Ok(signature)
    }

    /// Buys tokens from a bonding curve by spending SOL
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint to buy
    /// * `amount_sol` - Amount of SOL to spend in lamports
    /// * `slippage_basis_points` - Optional maximum acceptable slippage in basis points (1 bp = 0.01%). Defaults to 500
    /// * `priority_fee` - Optional priority fee configuration for compute units
    ///
    /// # Returns
    ///
    /// Returns the transaction signature if successful, or a ClientError if the operation fails
    pub async fn buy(
        &self,
        mint: &Pubkey,
        amount_sol: u64,
        slippage_basis_points: Option<u64>,
        priority_fee: Option<PriorityFee>,
    ) -> Result<Signature, error::ClientError> {
        // Get accounts and calculate buy amounts
        let global_account = self.get_global_account()?;
        let bonding_curve_account = self.get_bonding_curve_account(mint)?;
        let buy_amount = bonding_curve_account
            .get_buy_price(amount_sol)
            .map_err(error::ClientError::BondingCurveError)?;
        let buy_amount_with_slippage =
            utils::calculate_with_slippage_buy(amount_sol, slippage_basis_points.unwrap_or(500));

        // Get the recent blockhash
        let recent_blockhash = match fetch_recent_blockhash(&self.rpc).await {
            Ok(blockhash) => blockhash,
            Err(e) => return  Err(error::ClientError::SolanaClientError(e)),
        };

        // Instructions vector to collect all the instructions
        let mut instructions: Vec<Instruction> = Vec::new();

        // Add priority fee if provided
        if let Some(fee) = priority_fee {
            if let Some(limit) = fee.limit {
                let limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(limit);
                instructions.push(limit_ix);
            }

            if let Some(price) = fee.price {
                let price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
                instructions.push(price_ix);
            }
        }

        // Create Associated Token Account if needed
        let ata: Pubkey = get_associated_token_address(&self.payer.pubkey(), mint);
        if self.rpc.get_account(&ata).is_err() {
            let ata_ix = create_associated_token_account(
                &self.payer.pubkey(),
                &self.payer.pubkey(),
                mint,
                &constants::accounts::TOKEN_PROGRAM,
            );
            instructions.push(ata_ix);
        }

        // Add the buy instruction
        let buy_args = instruction::BuyInstructionArgs {
            amount: buy_amount,
            max_sol_cost: buy_amount_with_slippage,
        };

        let buy_ix = instruction::buy(
            &self.payer,
            mint,
            &global_account.fee_recipient,
            buy_args,
        );
        instructions.push(buy_ix);

        // Build the transaction
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&self.payer.pubkey()));

        // Sign the transaction
        let signers: Vec<&dyn solana_sdk::signer::Signer> = vec![&self.payer];
        transaction.sign(&signers, recent_blockhash);

        // Send the transaction
        let signature = self.rpc.send_and_confirm_transaction(&transaction).map_err(|e| {
            error::ClientError::SolanaClientError(e)
        })?;

        Ok(signature)
    }

    /// Sells tokens back to the bonding curve in exchange for SOL
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint to sell
    /// * `amount_token` - Optional amount of tokens to sell in base units. If None, sells entire balance
    /// * `slippage_basis_points` - Optional maximum acceptable slippage in basis points (1 bp = 0.01%). Defaults to 500
    /// * `priority_fee` - Optional priority fee configuration for compute units
    ///
    /// # Returns
    ///
    /// Returns the transaction signature if successful, or a ClientError if the operation fails
    // pub async fn sell(
    //     &self,
    //     mint: &Pubkey,
    //     amount_token: Option<u64>,
    //     slippage_basis_points: Option<u64>,
    //     priority_fee: Option<PriorityFee>,
    // ) -> Result<Signature, error::ClientError> {
    //     // Get the associated token address
    //     let ata: Pubkey = get_associated_token_address(&self.payer.pubkey(), mint);

    //     // Fetch balance
    //     let balance = self.rpc.get_token_account_balance(&ata).unwrap();
    //     let balance_u64: u64 = balance.amount.parse::<u64>().unwrap();

    //     // Use the provided amount or the full balance
    //     let _amount = amount_token.unwrap_or(balance_u64);

    //     // Get global account and bonding curve account
    //     let global_account = self.get_global_account()?;
    //     let bonding_curve_account = self.get_bonding_curve_account(mint)?;

    //     // Calculate minimum SOL output
    //     let min_sol_output = bonding_curve_account
    //         .get_sell_price(_amount, global_account.fee_basis_points)
    //         .map_err(error::ClientError::BondingCurveError)?;

    //     // Calculate min SOL output with slippage
    //     let _min_sol_output = utils::calculate_with_slippage_sell(
    //         min_sol_output,
    //         slippage_basis_points.unwrap_or(500),
    //     );

    //     // Set up RPC Client
    //     let self.rpc = SolanaRpcClient::new(self.rpc_url.clone());

    //     // Get the recent blockhash
    //     let recent_blockhash = self.fetch_recent_blockhash(&self.rpc).await?;

    //     // Instructions vector to collect all instructions
    //     let mut instructions: Vec<Instruction> = Vec::new();

    //     // Add priority fee if provided
    //     if let Some(fee) = priority_fee {
    //         if let Some(limit) = fee.limit {
    //             let limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(limit);
    //             instructions.push(limit_ix);
    //         }

    //         if let Some(price) = fee.price {
    //             let price_ix = ComputeBudgetInstruction::set_compute_unit_price(price);
    //             instructions.push(price_ix);
    //         }
    //     }

    //     // Create Associated Token Account if needed
    //     if self.rpc.get_account(&ata).is_err() {
    //         let ata_ix = create_associated_token_account(
    //             &self.payer.pubkey(),
    //             &self.payer.pubkey(),
    //             mint,
    //             &constants::accounts::TOKEN_PROGRAM,
    //         );
    //         instructions.push(ata_ix);
    //     }

    //     // Add the sell instruction
    //     let sell_args = instruction::SellInstructionArgs {
    //         amount: _amount,
    //         min_sol_output: _min_sol_output,
    //     };

    //     let sell_ix = sell(
    //         &self.payer,
    //         mint,
    //         &global_account.fee_recipient,
    //         sell_args,
    //     );
    //     instructions.push(sell_ix);

    //     // Build the transaction
    //     let mut transaction = Transaction::new_with_payer(&instructions, Some(&self.payer.pubkey()));

    //     // Sign the transaction
    //     let signers: Vec<&dyn solana_sdk::signer::Signer> = vec![&self.payer];
    //     transaction.sign(&signers, recent_blockhash);

    //     // Send the transaction
    //     let signature = self.rpc.send_and_confirm_transaction(&transaction).await.map_err(|e| {
    //         error::ClientError::SolanaClientError(e.to_string())
    //     })?;

    //     Ok(signature)
    // }

    /// Gets the Program Derived Address (PDA) for the global state account
    ///
    /// # Returns
    ///
    /// Returns the PDA public key derived from the GLOBAL_SEED
    pub fn get_global_pda() -> Pubkey {
        let seeds: &[&[u8]; 1] = &[constants::seeds::GLOBAL_SEED];
        let program_id: &Pubkey = &PROGRAM_ID;
        Pubkey::find_program_address(seeds, program_id).0
    }

    /// Gets the Program Derived Address (PDA) for the mint authority
    ///
    /// # Returns
    ///
    /// Returns the PDA public key derived from the MINT_AUTHORITY_SEED
    pub fn get_mint_authority_pda() -> Pubkey {
        let seeds: &[&[u8]; 1] = &[constants::seeds::MINT_AUTHORITY_SEED];
        let program_id: &Pubkey = &PROGRAM_ID;
        Pubkey::find_program_address(seeds, program_id).0
    }

    /// Gets the Program Derived Address (PDA) for a token's bonding curve account
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint
    ///
    /// # Returns
    ///
    /// Returns Some(PDA) if derivation succeeds, or None if it fails
    pub fn get_bonding_curve_pda(mint: &Pubkey) -> Option<Pubkey> {
        let seeds: &[&[u8]; 2] = &[constants::seeds::BONDING_CURVE_SEED, mint.as_ref()];
        let program_id: &Pubkey = &PROGRAM_ID;
        let pda: Option<(Pubkey, u8)> = Pubkey::try_find_program_address(seeds, program_id);
        pda.map(|pubkey| pubkey.0)
    }

    /// Gets the Program Derived Address (PDA) for a token's metadata account
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint
    ///
    /// # Returns
    ///
    /// Returns the PDA public key for the token's metadata account
    pub fn get_metadata_pda(mint: &Pubkey) -> Pubkey {
        let seeds: &[&[u8]; 3] = &[
            constants::seeds::METADATA_SEED,
            constants::accounts::MPL_TOKEN_METADATA.as_ref(),
            mint.as_ref(),
        ];
        let program_id: &Pubkey = &constants::accounts::MPL_TOKEN_METADATA;
        Pubkey::find_program_address(seeds, program_id).0
    }

    /// Gets the global state account data containing program-wide configuration
    ///
    /// # Returns
    ///
    /// Returns the deserialized GlobalAccount if successful, or a ClientError if the operation fails
    pub fn get_global_account(&self) -> Result<accounts::GlobalAccount, error::ClientError> {
        let global: Pubkey = Self::get_global_pda();

        let account = self
            .rpc
            .get_account(&global)
            .map_err(error::ClientError::SolanaClientError)?;

        accounts::GlobalAccount::try_from_slice(&account.data)
            .map_err(error::ClientError::BorshError)
    }

    /// Gets a token's bonding curve account data containing pricing parameters
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint
    ///
    /// # Returns
    ///
    /// Returns the deserialized BondingCurveAccount if successful, or a ClientError if the operation fails
    pub fn get_bonding_curve_account(
        &self,
        mint: &Pubkey,
    ) -> Result<accounts::BondingCurveAccount, error::ClientError> {
        let bonding_curve_pda =
            Self::get_bonding_curve_pda(mint).ok_or(error::ClientError::BondingCurveNotFound)?;

        let account = self
            .rpc
            .get_account(&bonding_curve_pda)
            .map_err(error::ClientError::SolanaClientError)?;

        accounts::BondingCurveAccount::try_from_slice(&account.data)
            .map_err(error::ClientError::BorshError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_client::solana_sdk::signer::keypair::Keypair;

    #[test]
    fn test_new_client() {
        let payer = Keypair::new();
        let client = PumpFun::new(Cluster::Devnet, &payer, None, None);
        assert_eq!(client.payer.pubkey(), payer.pubkey());
    }

    #[test]
    fn test_get_pdas() {
        let mint = Keypair::new();
        let global_pda = PumpFun::get_global_pda();
        let mint_authority_pda = PumpFun::get_mint_authority_pda();
        let bonding_curve_pda = PumpFun::get_bonding_curve_pda(&mint.pubkey());
        let metadata_pda = PumpFun::get_metadata_pda(&mint.pubkey());

        assert!(global_pda != Pubkey::default());
        assert!(mint_authority_pda != Pubkey::default());
        assert!(bonding_curve_pda.is_some());
        assert!(metadata_pda != Pubkey::default());
    }
}
