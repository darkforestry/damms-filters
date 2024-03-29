use std::sync::Arc;

use damms::{
    amm::{factory::Factory, AutomatedMarketMaker, AMM},
    errors::DAMMError,
};
use ethers::{
    providers::Middleware,
    types::{H160, U256},
};
use spinoff::{spinners, Color, Spinner};

use crate::batch_requests;

pub const U256_10_POW_18: U256 = U256([1000000000000000000, 0, 0, 0]);
pub const U256_10_POW_6: U256 = U256([1000000, 0, 0, 0]);

//Filter that removes AMMs with that contain less than a specified usd value
pub async fn filter_amms_below_usd_threshold<M: Middleware>(
    amms: Vec<AMM>,
    factories: &[Factory],
    usd_weth_pool: AMM, //TODO: could make this f64 and just pass in price?
    usd_value_in_pool_threshold: f64, // This is the threshold where we will filter out any pool with less value than this
    weth: H160,
    weth_value_in_token_to_weth_pool_threshold: U256, //This is the threshold where we will ignore any token price < threshold during batch calls
    step: usize,
    middleware: Arc<M>,
) -> Result<Vec<AMM>, DAMMError<M>> {
    let spinner = Spinner::new(
        spinners::Dots,
        "Filtering AMMs below USD threshold...",
        Color::Blue,
    );

    let weth_usd_price = usd_weth_pool.calculate_price(weth)?;

    //Init a new vec to hold the filtered AMMs
    let mut filtered_amms = vec![];

    let weth_values_in_pools = get_weth_values_in_amms(
        &amms,
        factories,
        weth,
        weth_value_in_token_to_weth_pool_threshold,
        step,
        middleware,
    )
    .await?;

    for (i, weth_value) in weth_values_in_pools.iter().enumerate() {
        if (weth_value / U256_10_POW_18).as_u64() as f64 * weth_usd_price
            >= usd_value_in_pool_threshold
        {
            //TODO: using clone for now since we only do this once but find a better way in a future update
            filtered_amms.push(amms[i].clone());
        }
    }

    spinner.success("All AMMs filtered");
    Ok(filtered_amms)
}

//Filter that removes AMMs with that contain less than a specified weth value
//
pub async fn filter_amms_below_weth_threshold<M: Middleware>(
    amms: Vec<AMM>,
    factories: &[Factory],
    weth: H160,
    weth_value_in_pool_threshold: U256, // This is the threshold where we will filter out any pool with less value than this
    weth_value_in_token_to_weth_pool_threshold: U256, //This is the threshold where we will ignore any token price < threshold during batch calls
    step: usize,
    middleware: Arc<M>,
) -> Result<Vec<AMM>, DAMMError<M>> {
    let spinner = Spinner::new(
        spinners::Dots,
        "Filtering AMMs below weth threshold...",
        Color::Blue,
    );

    let mut filtered_amms = vec![];

    let weth_values_in_pools = get_weth_values_in_amms(
        &amms,
        factories,
        weth,
        weth_value_in_token_to_weth_pool_threshold,
        step,
        middleware,
    )
    .await?;

    for (i, weth_value) in weth_values_in_pools.iter().enumerate() {
        if *weth_value >= weth_value_in_pool_threshold {
            //TODO: using clone for now since we only do this once but find a better way in a future update
            filtered_amms.push(amms[i].clone());
        }
    }

    spinner.success("All AMMs filtered");
    Ok(filtered_amms)
}

pub async fn get_weth_values_in_amms<M: Middleware>(
    amms: &[AMM],
    factories: &[Factory],
    weth: H160,
    weth_value_in_token_to_weth_pool_threshold: U256,
    step: usize,
    middleware: Arc<M>,
) -> Result<Vec<U256>, DAMMError<M>> {
    //Init a new vec to hold the filtered pools
    let mut aggregate_weth_values_in_amms = vec![];

    let mut idx_from = 0;
    let mut idx_to = if step > amms.len() { amms.len() } else { step };

    //TODO: see if you can just step by the pools rather than some index
    for _ in (0..amms.len()).step_by(step) {
        let weth_values_in_amms =
            batch_requests::filter_by_value::get_weth_value_in_amm_batch_request(
                &amms[idx_from..idx_to],
                factories,
                weth,
                weth_value_in_token_to_weth_pool_threshold,
                middleware.clone(),
            )
            .await?;

        //add weth values in pools to the aggregate array
        aggregate_weth_values_in_amms.extend(weth_values_in_amms);

        idx_from = idx_to;

        if idx_to + step > amms.len() {
            idx_to = amms.len() - 1
        } else {
            idx_to += step;
        }
    }

    Ok(aggregate_weth_values_in_amms)
}
