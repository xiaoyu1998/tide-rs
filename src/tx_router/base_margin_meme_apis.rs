use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use hex;

pub const BASE_SEPOLIA : &str = "https://base-sepolia.g.alchemy.com/v2/78EX3W8zQaMXPMs1RPt3nhTDefmKkuEB";
use alloy::{
    network::{EthereumWallet, Ethereum},
    signers::local::PrivateKeySigner,
    providers::ProviderBuilder, 
};

pub async fn buy(
    meme: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {
    
   let signer: PrivateKeySigner = args.private_key.parse().expect("should parse private key");
   let wallet = EthereumWallet::from(signer.clone());
   let rpc = (&args.rpc).parse()?;
   let client = ProviderBuilder::new().with_cached_nonce_management().wallet(wallet.clone()).on_http(rpc);

   let owner = utils::get_owner();
   let data_store_address = addresses::get("DataStore");
   let exchange_router_address = addresses::get("ExchangeRouter");
   let base_address = addresses::get_address("BASE");
   let meme_address = Address.from(meme);
   let pools = await getPools(data_store_address, [hash_pool_key(base_address, meme_address)]);
   let pool = pools[0];
   let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone());

   let base_decimals = pool.decimals;
   let amount0_in = (amount * base_decimals as f64) as U256;
   let amount1_out = (amount / price_limit) as U256;

   let params_send_tokens: SwapUtils.sendTokensCall = {
        token: base_address,
        receiver: owner.address,
        amount0In: amount0_in,
    };

   let params_swap: SwapUtils.SwapParams = {
        token0: base_address,
        token1: meme_address,
        amount0In: amount0_in,
        amount1In: U256::ZERO,
        amount0Out: U256::ZERO,
        amount1Out: amount1_out,
        to: owner.address,
    };

    let multicallArgs = [
        params_send_tokens.encode(),
        params_swap.encode(),
    ];

    let call_build = exchange_router.multicall(multicallArgs);
    let mut tx = call_build.into_transaction_request();
    let _ = execute(tx).await?;

    Ok(()) 

}

pub async fn sell(
    meme: String,
    amount: f64
) -> Result<(), String> {

    Ok(()) 

}