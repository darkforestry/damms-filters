use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use cfmms::{dex::Dex, errors::CFMMError, pool::Pool};
use ethers::{
    providers::Middleware,
    types::{H160, U256},
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use num_bigfloat::BigFloat;

use crate::batch_requests;

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
    dexes: &[Dex],
    weth: H160,
    weth_in_pool_threshold: U256,
    middleware: Arc<M>,
) -> Result<Vec<Pool>, CFMMError<M>> {
    let mut filtered_pools = vec![];

    let weth_values_in_pools =
        get_weth_values_in_pools(&pools, dexes, weth, weth_in_pool_threshold, middleware).await?;

    let mut i = 0;
    for weth_value in weth_values_in_pools {
        if weth_value >= weth_in_pool_threshold {
            filtered_pools.push(pools[i]);
        }

        i += 1;
    }

    Ok(filtered_pools)
}

pub async fn get_weth_values_in_pools<M: Middleware>(
    pools: &[Pool],
    dexes: &[Dex],
    weth: H160,
    weth_in_pool_threshold: U256,
    middleware: Arc<M>,
) -> Result<Vec<U256>, CFMMError<M>> {
    let multi_progress_bar = MultiProgress::new();
    let progress_bar = multi_progress_bar.add(ProgressBar::new(0));
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} {bar:40.cyan/blue} {pos:>7}/{len:7} Pools Filtered")
            .unwrap()
            .progress_chars("##-"),
    );

    progress_bar.set_length(pools.len() as u64);
    progress_bar.set_message("Getting weth value in pools: ");

    //Init a new vec to hold the filtered pools
    let mut aggregate_weth_values_in_pools = vec![];

    let step = 766; //max batch size for this call until codesize is too large
    let mut idx_from = 0;
    let mut idx_to = if step > pools.len() {
        pools.len()
    } else {
        step
    };

    for _ in (0..pools.len()).step_by(step) {
        let weth_values_in_pools =
            batch_requests::filter_by_value::get_weth_value_in_pool_batch_request(
                &pools[idx_from..idx_to],
                dexes,
                weth,
                weth_in_pool_threshold,
                middleware.clone(),
            )
            .await?;

        //add weth values in pools to the aggregate array
        aggregate_weth_values_in_pools.extend(weth_values_in_pools);

        idx_from = idx_to;

        if idx_to + step > pools.len() {
            idx_to = pools.len() - 1
        } else {
            idx_to = idx_to + step;
        }

        progress_bar.inc(step as u64);
    }

    Ok(aggregate_weth_values_in_pools)
}
