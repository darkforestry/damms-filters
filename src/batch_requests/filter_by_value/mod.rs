use dcfmms::{dex::Dex, pool::Pool};
use ethers::{
    abi::{ParamType, Token},
    prelude::abigen,
    providers::Middleware,
    types::{Bytes, H160, U256},
};
use std::sync::Arc;

abigen!(
    GetWethValueInPoolBatchRequest,
    "src/batch_requests/filter_by_value/GetWethValueInPoolBatchRequest.json";
);

pub async fn get_weth_value_in_pool_batch_request<M: Middleware>(
    pools: &[Pool],
    dexes: &[Dex],
    weth: H160,
    weth_value_in_token_to_weth_pool_threshold: U256,
    middleware: Arc<M>,
) -> Result<Vec<U256>, dcfmms::errors::CFMMError<M>> {
    let mut weth_values_in_pools = vec![];

    let pools = pools
        .iter()
        .map(|p| Token::Address(p.address()))
        .collect::<Vec<Token>>();

    let dex_is_uni_v3 = dexes
        .iter()
        .map(|d| match d {
            Dex::UniswapV2(_) => Token::Bool(false),
            Dex::UniswapV3(_) => Token::Bool(true),
        })
        .collect::<Vec<Token>>();

    let dexes = dexes
        .iter()
        .map(|d| Token::Address(d.factory_address()))
        .collect::<Vec<Token>>();

    let constructor_args = Token::Tuple(vec![
        Token::Array(pools),
        Token::Array(dexes),
        Token::Array(dex_is_uni_v3),
        Token::Address(weth),
        Token::Uint(weth_value_in_token_to_weth_pool_threshold),
    ]);

    let deployer = GetWethValueInPoolBatchRequest::deploy(middleware, constructor_args).unwrap();
    let return_data: Bytes = deployer.call_raw().await?;

    let return_data_tokens = ethers::abi::decode(
        &[ParamType::Array(Box::new(ParamType::Uint(256)))],
        &return_data,
    )?;

    for token_array in return_data_tokens {
        if let Some(arr) = token_array.into_array() {
            for token in arr {
                if let Some(weth_value_in_pool) = token.into_uint() {
                    weth_values_in_pools.push(weth_value_in_pool);
                }
            }
        }
    }

    Ok(weth_values_in_pools)
}
