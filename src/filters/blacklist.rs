use std::collections::HashSet;

use cfmms::pool::Pool;
use ethers::types::H160;

use super::get_tokens_from_pool;

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
