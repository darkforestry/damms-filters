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
    //Get all pools

    //Init a new vec to hold the filtered pools
    let mut filtered_pools = vec![];

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

    Ok(filtered_pools)
}
