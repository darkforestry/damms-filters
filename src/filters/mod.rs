pub mod blacklist;
pub mod value;

use async_trait::async_trait;
use cfmms::dex::Dex;
use cfmms::errors::CFMMError;
use cfmms::pool::{Pool, UniswapV2Pool, UniswapV3Pool};
use ethers::providers::Middleware;
use ethers::types::H160;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use num_bigfloat::BigFloat;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{collections::HashSet, sync::Arc};

pub fn get_tokens_from_pool(pool: &Pool) -> Vec<H160> {
    match pool {
        Pool::UniswapV2(pool) => {
            vec![pool.token_a, pool.token_b]
        }
        Pool::UniswapV3(pool) => {
            vec![pool.token_a, pool.token_b]
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
