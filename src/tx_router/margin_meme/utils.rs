use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use hex;



pub const BASE_SEPOLIA : &str = "https://base-sepolia.g.alchemy.com/v2/78EX3W8zQaMXPMs1RPt3nhTDefmKkuEB";


fn hex_to_pubkey(mint_str: &String) -> Result<Pubkey, String> {
    // Convert the string to a 32-byte array
    let mint_bytes = match hex::decode(mint_str.to_string()) {
        Ok(bytes) if bytes.len() == 32 => bytes,
        Ok(_) => return Err("Invalid pubkey length".to_string()),
        Err(err) => return Err(format!("Failed to decode pubkey string: {}", err)),
    };

    // Create Pubkey from the 32-byte array
    let pubkey = Pubkey::new_from_array(mint_bytes.try_into().unwrap());
    Ok(pubkey)
}

pub async fn exeute(
    tx: String,
) -> Result<(), String> {

}

pub async fn buy(
    meme: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {
   let signer: PrivateKeySigner = args.private_key.parse().expect("should parse private key");
   let wallet = EthereumWallet::from(signer.clone());
   let rpc = (&args.rpc).parse()?;
   let client = ProviderBuilder::new().with_cached_nonce_management().wallet(wallet.clone()).on_http(rpc);

   let owner = get_owner();
   let base_address = get_address("BASE");
   let meme_address = Address.from(meme);
   let data_store_address = get_address("DataStore");
   let exchange_router_address = get_address("ExchangeRouter");
   let pools = await getPools(data_store_address, [hash_pool_key(base_address, meme_address)]);
   let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone());

   let base_decimals = pools[0].decimals;
   let amount0_in = (amount * base_decimals as f64) as U256;
   let amount1_out = (amount / price_limitas) as U256;

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

