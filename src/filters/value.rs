use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use cfmms::{dex::Dex, errors::CFMMError, pool::Pool};
use ethers::{providers::Middleware, types::H160};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use num_bigfloat::BigFloat;

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
        let total_usd_value_in_pool = match get_weth_value_in_pool(
            &pool,
            weth_address,
            &dexes,
            token_weth_pool_min_weth_threshold,
            token_weth_prices.clone(),
            middleware.clone(),
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

                CFMMError::PoolDataError => {
                    println!("PoolDataError");
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
        let total_weth_value_in_pool = match get_weth_value_in_pool(
            &pool,
            weth_address,
            &dexes,
            token_weth_pool_min_weth_threshold,
            token_weth_prices.clone(),
            middleware.clone(),
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

//TODO:FIXME:

// basically to get the value of a pool in usd, we need to get the pool's value in weth, then multiply that by the price of usd per weth
//So really we only need to create a batch function to get value of weth in the pool
//To do this, we can create a constructor that takes in any amount of dexes, v2 or v3.
// Then if the token in the pool is weth, we record that value. If the token is not weth, we search each of the dexes for a pair for the token/weth. Once we find all the token/weth pairs, we get the one with the best liquidity
//Then we calculate calculate the price of token/weth. We then use exchange rate and multiply the amount of the token in the pool to get the amount of weth in the pool. Then we add the amount of weth in the pool to a list and after iterating
//Through a certain amount of pools (whatever the max is), we return the list. Then offchain, we simply multiply the returned value by the price of weth/usd which gives us the value of the pool in usd.

//TODO: batch this
async fn get_weth_value_in_pool<M: Middleware>(
    pool: &Pool,
    weth_address: H160,
    dexes: &[Dex],
    token_weth_pool_min_weth_threshold: u128,
    token_weth_prices: Arc<Mutex<HashMap<H160, f64>>>,
    middleware: Arc<M>,
) -> Result<f64, CFMMError<M>> {
    match pool {
        Pool::UniswapV2(pool) => {
            let token_a_price_per_weth = token_weth_prices
                .lock()
                .unwrap()
                .get(&pool.token_a)
                .map(|price| price.to_owned());

            let token_a_price_per_weth = match token_a_price_per_weth {
                Some(price) => price,
                None => {
                    let price = get_price_of_token_per_weth(
                        pool.token_a,
                        weth_address,
                        dexes,
                        token_weth_pool_min_weth_threshold,
                        middleware.clone(),
                    )
                    .await?;

                    token_weth_prices
                        .lock()
                        .unwrap()
                        .insert(pool.token_a, price);

                    price
                }
            };

            //Get weth value of token a in pool
            let token_a_weth_value_in_pool = BigFloat::from(pool.reserve_0).to_f64()
                / 10f64.powf(pool.token_a_decimals.into())
                / token_a_price_per_weth;

            let token_b_price_per_weth = token_weth_prices
                .lock()
                .unwrap()
                .get(&pool.token_b)
                .map(|price| price.to_owned());

            let token_b_price_per_weth = match token_b_price_per_weth {
                Some(price) => price.to_owned(),
                None => {
                    let price = get_price_of_token_per_weth(
                        pool.token_b,
                        weth_address,
                        dexes,
                        token_weth_pool_min_weth_threshold,
                        middleware.clone(),
                    )
                    .await?;

                    token_weth_prices
                        .lock()
                        .unwrap()
                        .insert(pool.token_b, price);

                    price
                }
            };

            //Get weth value of token a in pool
            let token_b_weth_value_in_pool = BigFloat::from(pool.reserve_1).to_f64()
                / 10f64.powf(pool.token_b_decimals.into())
                / token_b_price_per_weth;

            //Return weth value in pool
            Ok(token_a_weth_value_in_pool + token_b_weth_value_in_pool)
        }

        Pool::UniswapV3(pool) => {
            let (reserve_0, reserve_1) = pool.calculate_virtual_reserves();

            let token_a_price_per_weth = token_weth_prices
                .lock()
                .unwrap()
                .get(&pool.token_a)
                .map(|price| price.to_owned());

            let token_a_price_per_weth = match token_a_price_per_weth {
                Some(price) => price,
                None => {
                    let price = get_price_of_token_per_weth(
                        pool.token_a,
                        weth_address,
                        dexes,
                        token_weth_pool_min_weth_threshold,
                        middleware.clone(),
                    )
                    .await?;

                    token_weth_prices
                        .lock()
                        .unwrap()
                        .insert(pool.token_a, price);

                    price
                }
            };

            //Get weth value of token a in pool
            let token_a_weth_value_in_pool = BigFloat::from(reserve_0).to_f64()
                / 10f64.powf(pool.token_a_decimals.into())
                / token_a_price_per_weth;

            let token_b_price_per_weth = token_weth_prices
                .lock()
                .unwrap()
                .get(&pool.token_b)
                .map(|price| price.to_owned());

            let token_b_price_per_weth = match token_b_price_per_weth {
                Some(price) => price.to_owned(),
                None => {
                    let price = get_price_of_token_per_weth(
                        pool.token_b,
                        weth_address,
                        dexes,
                        token_weth_pool_min_weth_threshold,
                        middleware.clone(),
                    )
                    .await?;

                    token_weth_prices
                        .lock()
                        .unwrap()
                        .insert(pool.token_b, price);

                    price
                }
            };

            //Get weth value of token a in pool
            let token_b_weth_value_in_pool = BigFloat::from(reserve_1).to_f64()
                / 10f64.powf(pool.token_b_decimals.into())
                / token_b_price_per_weth;

            //Return weth value in pool
            Ok(token_a_weth_value_in_pool + token_b_weth_value_in_pool)
        }
    }
}

//Gets the best token to weth pairing from the dexes provided
async fn get_token_to_weth_pool<M: Middleware>(
    token_a: H160,
    weth_address: H160,
    dexes: &[Dex],
    token_weth_pool_min_weth_threshold: u128,
    middleware: Arc<M>,
) -> Result<Pool, CFMMError<M>> {
    let _pair_address = H160::zero();
    let mut _pool: Pool;

    let mut best_pool: Option<Pool> = None;
    let mut best_weth_reserves = 0_u128;

    for dex in dexes {
        match dex
            .get_pool_with_best_liquidity(token_a, weth_address, middleware.clone())
            .await
        {
            Ok(pool) => {
                if pool.is_some() {
                    match pool.unwrap() {
                        Pool::UniswapV2(univ2_pool) => {
                            if univ2_pool.token_a == weth_address {
                                if univ2_pool.reserve_0 > best_weth_reserves {
                                    best_weth_reserves = univ2_pool.reserve_0;
                                    best_pool = pool;
                                } else if univ2_pool.reserve_1 > best_weth_reserves {
                                    best_weth_reserves = univ2_pool.reserve_1;
                                    best_pool = pool;
                                }
                            } else if univ2_pool.reserve_1 > best_weth_reserves {
                                best_weth_reserves = univ2_pool.reserve_1;
                                best_pool = pool;
                            } else if univ2_pool.reserve_0 > best_weth_reserves {
                                best_weth_reserves = univ2_pool.reserve_0;
                                best_pool = pool;
                            }
                        }

                        Pool::UniswapV3(univ3_pool) => {
                            let (reserve_0, reserve_1) = univ3_pool.calculate_virtual_reserves();

                            if univ3_pool.token_a == weth_address {
                                if reserve_0 > best_weth_reserves {
                                    best_weth_reserves = reserve_0;
                                    best_pool = pool;
                                } else if reserve_1 > best_weth_reserves {
                                    best_weth_reserves = reserve_1;
                                    best_pool = pool;
                                }
                            } else if reserve_1 > best_weth_reserves {
                                best_weth_reserves = reserve_1;
                                best_pool = pool;
                            } else if reserve_0 > best_weth_reserves {
                                best_weth_reserves = reserve_0;
                                best_pool = pool;
                            }
                        }
                    }
                }
            }

            Err(pair_sync_error) => match pair_sync_error {
                CFMMError::ContractError(_) => continue,
                other => return Err(other),
            },
        };
    }

    //If the pool getting the price doesnt have at least x eth, return no pair
    if best_weth_reserves >= token_weth_pool_min_weth_threshold {
        Ok(best_pool.unwrap())
    } else {
        Err(CFMMError::PairDoesNotExistInDexes(token_a, weth_address))
    }
}
