use std::{error::Error, str::FromStr, sync::Arc};

use ethers::{
    providers::{Http, Provider},
    types::{H160, U256},
};

use cfmms::{
    dex::{Dex, DexVariant},
    pool::{Pool, UniswapV2Pool},
    sync,
};

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    //Add rpc endpoint here:
    let rpc_endpoint =
        std::env::var("POLYGON_MAINNET_ENDPOINT").expect("Could not get POLYGON_MAINNET_ENDPOINT");
    let provider = Arc::new(Provider::<Http>::try_from(rpc_endpoint).unwrap());

    let dexes = vec![
        //Quickswap
        Dex::new(
            H160::from_str("0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32").unwrap(),
            DexVariant::UniswapV2,
            4931780,
        ),
        // Add Sushiswap
        Dex::new(
            H160::from_str("0xc35DADB65012eC5796536bD9864eD8773aBc74C4").unwrap(),
            DexVariant::UniswapV2,
            11333218,
        ),
        //Add apeswap
        Dex::new(
            H160::from_str("0xCf083Be4164828f00cAE704EC15a36D711491284").unwrap(),
            DexVariant::UniswapV2,
            15298801,
        ),
        //Add uniswap v3
        Dex::new(
            H160::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984").unwrap(),
            DexVariant::UniswapV3,
            22757547,
        ),
    ];

    //Sync pools
    let pools =
        sync::sync_pairs_with_throttle(dexes.clone(), 100000, provider.clone(), 7, None).await?;

    //Create a list of blacklisted tokens
    let blacklisted_tokens =
        vec![H160::from_str("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984").unwrap()];

    //Filter out blacklisted tokens
    let filtered_pools =
        cfmms_pool_filters::filters::address::filter_blacklisted_tokens(pools, blacklisted_tokens);

    let weth_address = H160::from_str("0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270").unwrap();
    let usd_weth_pair_address =
        H160::from_str("0x6e7a5FAFcec6BB1e78bAE2A1F0B612012BF14827").unwrap();

    let usd_weth_pool = Pool::UniswapV2(
        UniswapV2Pool::new_from_address(usd_weth_pair_address, provider.clone()).await?,
    );

    let weth_value_in_token_to_weth_pool_threshold =
        U256::from_str("100000000000000000000").unwrap(); // 1000 matic

    println!("Filtering pools below usd threshold");

    let filtered_pools = cfmms_pool_filters::filters::value::filter_pools_below_usd_threshold(
        filtered_pools,
        &dexes,
        usd_weth_pool,
        30000.00, //Setting usd_threshold to 10000.00 filters out any pool that contains less than $30k USD value
        weth_address,
        // When getting token to weth price to determine weth value in pool, dont use price with weth reserves with less than $1000 USD worth
        weth_value_in_token_to_weth_pool_threshold,
        provider.clone(),
    )
    .await?;

    dbg!(filtered_pools.clone());
    dbg!(filtered_pools.len());

    Ok(())
}

pub const U256_2_POW_18: U256 = U256([2000000000000000000, 0, 0, 0]);
