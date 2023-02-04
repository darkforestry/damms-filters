use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use cfmms::{dex::Dex, errors::CFMMError, pool::Pool};
use ethers::{providers::Middleware, types::H160};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

//Filter that removes pools with that contain less than a specified usd value
#[allow(clippy::too_many_arguments)]
pub async fn filter_pools_below_usd_threshold<M: Middleware>(
    pools: Vec<Pool>,
    dexes: Vec<Dex>,
    usd_weth_pool: Pool,
    usd_address: H160,
    weth_address: H160,
    usd_threshold: f64,
    token_weth_pool_min_weth_threshold: u128,
    middleware: Arc<M>,
) -> Result<Vec<Pool>, CFMMError<M>> {
    let multi_progress_bar = MultiProgress::new();
    let progress_bar = multi_progress_bar.add(ProgressBar::new(0));
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7} Pools Filtered")
            .unwrap()
            .progress_chars("##-"),
    );

    progress_bar.set_length(pools.len() as u64);
    progress_bar.set_message("Filtering pools: ");

    //Init a new vec to hold the filtered pools
    let mut filtered_pools = vec![];

    //Get price of weth in USD
    let usd_price_per_weth = usd_weth_pool.calculate_price(usd_address)?;

    //Initialize a Hashmap to keep track of token/weth prices already found to avoid unnecessary calls to the node
    let token_weth_prices: Arc<Mutex<HashMap<H160, f64>>> = Arc::new(Mutex::new(HashMap::new()));
    //For each pool, check if the usd value meets the specified threshold
    for pool in pools {
        progress_bar.inc(1);
        //Compare the sum of token_a and token_b usd value against the specified threshold
        let total_usd_value_in_pool = match pool
            .get_weth_value_in_pool(
                weth_address,
                &dexes,
                token_weth_pool_min_weth_threshold,
                middleware.clone(),
                token_weth_prices.clone(),
            )
            .await
        {
            Ok(weth_value_in_pool) => weth_value_in_pool * usd_price_per_weth,
            Err(pair_sync_error) => match pair_sync_error {
                CFMMError::PairDoesNotExistInDexes(token_a, token_b) => {
                    println!("Pair does not exist in dexes: {token_a:?} {token_b:?}");
                    0.0
                }
                CFMMError::ContractError(contract_error) => {
                    println!("Contract Error: {contract_error:?}");

                    0.0
                }
                _ => return Err(pair_sync_error),
            },
        };

        if usd_threshold <= total_usd_value_in_pool {
            filtered_pools.push(pool);
        }
    }

    Ok(filtered_pools)
}

//Filter that removes pools with that contain less than a specified weth value
//
pub async fn filter_pools_below_weth_threshold<M: Middleware>(
    pools: Vec<Pool>,
    dexes: Vec<Dex>,
    weth_address: H160,
    weth_threshold: f64,
    token_weth_pool_min_weth_threshold: u128,
    middleware: Arc<M>,
) -> Result<Vec<Pool>, CFMMError<M>> {
    let multi_progress_bar = MultiProgress::new();
    let progress_bar = multi_progress_bar.add(ProgressBar::new(0));
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7} Pools Filtered")
            .unwrap()
            .progress_chars("##-"),
    );

    progress_bar.set_length(pools.len() as u64);
    progress_bar.set_message("Filtering pools: ");

    //Init a new vec to hold the filtered pools
    let mut filtered_pools = vec![];

    //Initialize a Hashmap to keep track of token/weth prices already found to avoid unnecessary calls to the node
    let token_weth_prices: Arc<Mutex<HashMap<H160, f64>>> = Arc::new(Mutex::new(HashMap::new()));
    //For each pool, check if the usd value meets the specified threshold
    for pool in pools {
        let token_weth_prices = token_weth_prices.clone();
        let middleware = middleware.clone();
        let dexes = dexes.clone();
        let progress_bar = progress_bar.clone();

        progress_bar.inc(1);
        //Compare the sum of token_a and token_b usd value against the specified threshold
        let total_weth_value_in_pool = match pool
            .get_weth_value_in_pool(
                weth_address,
                &dexes,
                token_weth_pool_min_weth_threshold,
                middleware.clone(),
                token_weth_prices.clone(),
                request_throttle.clone(),
            )
            .await
        {
            Ok(weth_value_in_pool) => weth_value_in_pool,
            Err(pair_sync_error) => match pair_sync_error {
                CFMMError::PairDoesNotExistInDexes(_, _) | CFMMError::ContractError(_) => 0.0,
                _ => return Err(pair_sync_error),
            },
        };

        if weth_threshold <= total_weth_value_in_pool {
            filtered_pools.push(pool);
        }
    }

    Ok(filtered_pools)
}

async fn get_price_of_token_per_weth<M: Middleware>(
    token_address: H160,
    weth_address: H160,
    dexes: &[Dex],
    token_weth_pool_min_weth_threshold: u128,
    middleware: Arc<M>,
) -> Result<f64, CFMMError<M>> {
    if token_address == weth_address {
        return Ok(1.0);
    }

    //Get token_a/weth price
    let token_weth_pool = get_token_to_weth_pool(
        token_address,
        weth_address,
        dexes,
        token_weth_pool_min_weth_threshold,
        middleware.clone(),
    )
    .await?;

    let token_price_per_weth = token_weth_pool.calculate_price(token_address);

    Ok(token_price_per_weth?)
}
