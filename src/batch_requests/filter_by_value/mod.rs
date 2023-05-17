use damms::amm::{
    factory::{AutomatedMarketMakerFactory, Factory},
    AutomatedMarketMaker, AMM,
};
use ethers::{
    abi::{ParamType, Token},
    prelude::abigen,
    providers::Middleware,
    types::{Bytes, H160, U256},
};
use std::sync::Arc;

abigen!(
    GetWethValueInAMMBatchRequest,
    "src/batch_requests/filter_by_value/GetWethValueInAMMBatchRequest.json";
);

pub async fn get_weth_value_in_amm_batch_request<M: Middleware>(
    amms: &[AMM],
    factories: &[Factory],
    weth: H160,
    weth_value_in_token_to_weth_pool_threshold: U256,
    middleware: Arc<M>,
) -> Result<Vec<U256>, damms::errors::DAMMError<M>> {
    let mut weth_values_in_pools = vec![];

    let amms = amms
        .iter()
        .map(|a| Token::Address(a.address()))
        .collect::<Vec<Token>>();

    let factory_is_uni_v3 = factories
        .iter()
        .map(|d| match d {
            Factory::UniswapV2Factory(_) => Token::Bool(false),
            Factory::UniswapV3Factory(_) => Token::Bool(true),
            Factory::IziSwapFactory(_) => Token::Bool(true) //TODO: This needs to be changed
        })
        .collect::<Vec<Token>>();

    let factories = factories
        .iter()
        .map(|f| Token::Address(f.address()))
        .collect::<Vec<Token>>();

    let constructor_args = Token::Tuple(vec![
        Token::Array(amms),
        Token::Array(factories),
        Token::Array(factory_is_uni_v3),
        Token::Address(weth),
        Token::Uint(weth_value_in_token_to_weth_pool_threshold),
    ]);

    let deployer = GetWethValueInAMMBatchRequest::deploy(middleware, constructor_args).unwrap();
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
