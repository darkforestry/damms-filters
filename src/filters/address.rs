use std::collections::{HashMap, HashSet};

use cfmms::pool::Pool;
use ethers::types::H160;
use regex::Regex;
use reqwest::Error;

//Filters out pools where the blacklisted address is the token_a address or token_b address
pub fn filter_blacklisted_tokens(pools: Vec<Pool>, blacklisted_addresses: Vec<H160>) -> Vec<Pool> {
    let mut filtered_pools = vec![];
    let blacklist: HashSet<H160> = blacklisted_addresses.into_iter().collect();

    for pool in pools {
        let mut blacklisted_token_in_pool = false;
        for token in get_tokens_from_pool(&pool) {
            if blacklist.contains(&token) {
                blacklisted_token_in_pool = true;
            }
        }

        if !blacklisted_token_in_pool {
            filtered_pools.push(pool);
        }
    }

    filtered_pools
}

//Filters out pools where the blacklisted address is the pair address
pub fn filter_blacklisted_pools(pools: Vec<Pool>, blacklisted_addresses: Vec<H160>) -> Vec<Pool> {
    let mut filtered_pools = vec![];
    let blacklist: HashSet<H160> = blacklisted_addresses.into_iter().collect();

    for pool in pools {
        if !blacklist.contains(&pool.address()) {
            filtered_pools.push(pool);
        }
    }

    filtered_pools
}

//Filters out pools where the blacklisted address is the pair address, token_a address or token_b address
pub fn filter_blacklisted_addresses(
    pools: Vec<Pool>,
    blacklisted_addresses: Vec<H160>,
) -> Vec<Pool> {
    let mut filtered_pools = vec![];
    let blacklist: HashSet<H160> = blacklisted_addresses.into_iter().collect();

    for pool in pools {
        let mut blacklisted_address_in_pool = false;
        for token in get_tokens_from_pool(&pool) {
            if blacklist.contains(&token) {
                blacklisted_address_in_pool = true;
            }
        }

        if blacklist.contains(&pool.address()) {
            blacklisted_address_in_pool = true;
        }

        if !blacklisted_address_in_pool {
            filtered_pools.push(pool);
        }
    }

    filtered_pools
}

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

// //Filters only pools that contain tokens from https://tokenlists.org/
// pub async fn filter_token_list_tokens(pools: Vec<Pool>) -> Result<Vec<Pool>, reqwest::Error> {
//     let mut filtered_pools = vec![];

//     //There is a better way to do this but this is fine and it only runs once
//     //This is also kind of hacky but it will work
//     //We get the json as a string then extract all of the addresses using regex, then once we have accumulated all of the addresses
//     //We can filter the list
//     let token_list_endpoints = vec!["", ""];

//     let mut token_list_addresses = vec![];

//     for endpoint in token_list_endpoints {
//         let resp = reqwest::get(endpoint).await?.json::<String>().await?;
//         let pattern = regex::Regex::new(r"\d+").unwrap();
//         // iterate over all matches
//         let endpoint_addresses = pattern
//             .find_iter(&resp)
//             .filter_map(|address| address.as_str().parse::<H160>().ok())
//             .collect::<Vec<H160>>();

//     }
//     Ok(filtered_pools)
// }
