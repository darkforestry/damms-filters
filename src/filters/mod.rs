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
