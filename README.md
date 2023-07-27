# damms-filters

**[Note: `damms-filters` has been migrated into `amms` and this repo has been archived.](https://github.com/darkforestry/amms-rs)**

A collection of filters to reduce invalid or poor quality AMMs on EVM chains.

## Value Filters
Note: A good `weth_value_in_token_to_weth_pool_threshold` is a setting with a $USD value > $1000 at least. Lower than this, you are getting pools that could have poor liquidity and show inflated prices.

```rust
pub async fn filter_pools_below_weth_threshold<M: Middleware>(
    pools: Vec<Pool>,
    dexes: &[Dex],
    weth: H160,
    weth_value_in_pool_threshold: U256, // This is the threshold where we will filter out any pool with less value than this
    weth_value_in_token_to_weth_pool_threshold: U256, //This is the threshold where we will ignore any token price < threshold during batch calls
    middleware: Arc<M>,
) -> Result<Vec<Pool>, CFMMError<M>> {
// --snip--
}
```


```rust
pub async fn filter_pools_below_usd_threshold<M: Middleware>(
    pools: Vec<Pool>,
    dexes: &[Dex],
    usd_weth_pool: Pool, 
    usd_value_in_pool_threshold: f64, // This is the threshold where we will filter out any pool with less value than this
    weth: H160,
    weth_value_in_token_to_weth_pool_threshold: U256, //This is the threshold where we will ignore any token price < threshold during batch calls
    middleware: Arc<M>,
) -> Result<Vec<Pool>, CFMMError<M>> {
// --snip--
}
```
