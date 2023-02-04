#[async_trait]
impl FilteredPool for Pool {
    fn address(&self) -> H160 {
        match self {
            Pool::UniswapV2(pool) => pool.address(),
            Pool::UniswapV3(pool) => pool.address(),
        }
    }

    fn tokens(&self) -> Vec<H160> {
        match self {
            Pool::UniswapV2(pool) => pool.tokens(),
            Pool::UniswapV3(pool) => pool.tokens(),
        }
    }

    async fn get_weth_value_in_pool<M: Middleware>(
        &self,
        weth_address: H160,
        dexes: &[Dex],
        token_weth_pool_min_weth_threshold: u128,
        middleware: Arc<M>,
        token_weth_prices: Arc<Mutex<HashMap<H160, f64>>>,
        request_throttle: Arc<Mutex<RequestThrottle>>,
    ) -> Result<f64, CFMMError<M>> {
        match self {
            Pool::UniswapV2(pool) => {
                pool.get_weth_value_in_pool(
                    weth_address,
                    dexes,
                    token_weth_pool_min_weth_threshold,
                    middleware,
                    token_weth_prices,
                    request_throttle,
                )
                .await
            }
            Pool::UniswapV3(pool) => {
                pool.get_weth_value_in_pool(
                    weth_address,
                    dexes,
                    token_weth_pool_min_weth_threshold,
                    middleware,
                    token_weth_prices,
                    request_throttle,
                )
                .await
            }
        }
    }
}

#[async_trait]
impl FilteredPool for UniswapV2Pool {
    fn address(&self) -> H160 {
        self.address
    }

    fn tokens(&self) -> Vec<H160> {
        vec![self.token_a, self.token_b]
    }

    async fn get_weth_value_in_pool<M: Middleware>(
        pool: &Pool,
        weth_address: H160,
        dexes: &[Dex],
        token_weth_pool_min_weth_threshold: u128,
        middleware: Arc<M>,
        token_weth_prices: Arc<Mutex<HashMap<H160, f64>>>,
        request_throttle: Arc<Mutex<RequestThrottle>>,
    ) -> Result<f64, CFMMError<M>> {
        let token_a_price_per_weth = token_weth_prices
            .lock()
            .unwrap()
            .get(&self.token_a)
            .map(|price| price.to_owned());

        let token_a_price_per_weth = match token_a_price_per_weth {
            Some(price) => price,
            None => {
                request_throttle.lock().unwrap().increment_or_sleep(1);
                let price = get_price_of_token_per_weth(
                    self.token_a,
                    weth_address,
                    dexes,
                    token_weth_pool_min_weth_threshold,
                    middleware.clone(),
                )
                .await?;

                token_weth_prices
                    .lock()
                    .unwrap()
                    .insert(self.token_a, price);

                price
            }
        };

        //Get weth value of token a in pool
        let token_a_weth_value_in_pool = BigFloat::from(self.reserve_0).to_f64()
            / 10f64.powf(self.token_a_decimals.into())
            / token_a_price_per_weth;

        let token_b_price_per_weth = token_weth_prices
            .lock()
            .unwrap()
            .get(&self.token_b)
            .map(|price| price.to_owned());

        let token_b_price_per_weth = match token_b_price_per_weth {
            Some(price) => price.to_owned(),
            None => {
                request_throttle.lock().unwrap().increment_or_sleep(1);
                let price = get_price_of_token_per_weth(
                    self.token_b,
                    weth_address,
                    dexes,
                    token_weth_pool_min_weth_threshold,
                    middleware.clone(),
                )
                .await?;

                token_weth_prices
                    .lock()
                    .unwrap()
                    .insert(self.token_b, price);

                price
            }
        };

        //Get weth value of token a in pool
        let token_b_weth_value_in_pool = BigFloat::from(self.reserve_1).to_f64()
            / 10f64.powf(self.token_b_decimals.into())
            / token_b_price_per_weth;

        //Return weth value in pool
        Ok(token_a_weth_value_in_pool + token_b_weth_value_in_pool)
    }
}

#[async_trait]
impl FilteredPool for UniswapV3Pool {
    fn address(&self) -> H160 {
        self.address
    }

    fn tokens(&self) -> Vec<H160> {
        vec![self.token_a, self.token_b]
    }

    async fn get_weth_value_in_pool<M: Middleware>(
        &self,
        weth_address: H160,
        dexes: &[Dex],
        token_weth_pool_min_weth_threshold: u128,
        middleware: Arc<M>,
        token_weth_prices: Arc<Mutex<HashMap<H160, f64>>>,
        request_throttle: Arc<Mutex<RequestThrottle>>,
    ) -> Result<f64, CFMMError<M>> {
        let (reserve_0, reserve_1) = self.calculate_virtual_reserves();

        let token_a_price_per_weth = token_weth_prices
            .lock()
            .unwrap()
            .get(&self.token_a)
            .map(|price| price.to_owned());

        let token_a_price_per_weth = match token_a_price_per_weth {
            Some(price) => price,
            None => {
                request_throttle.lock().unwrap().increment_or_sleep(1);
                let price = get_price_of_token_per_weth(
                    self.token_a,
                    weth_address,
                    dexes,
                    token_weth_pool_min_weth_threshold,
                    middleware.clone(),
                )
                .await?;

                token_weth_prices
                    .lock()
                    .unwrap()
                    .insert(self.token_a, price);

                price
            }
        };

        //Get weth value of token a in pool
        let token_a_weth_value_in_pool = BigFloat::from(reserve_0).to_f64()
            / 10f64.powf(self.token_a_decimals.into())
            / token_a_price_per_weth;

        let token_b_price_per_weth = token_weth_prices
            .lock()
            .unwrap()
            .get(&self.token_b)
            .map(|price| price.to_owned());

        let token_b_price_per_weth = match token_b_price_per_weth {
            Some(price) => price.to_owned(),
            None => {
                request_throttle.lock().unwrap().increment_or_sleep(1);
                let price = get_price_of_token_per_weth(
                    self.token_b,
                    weth_address,
                    dexes,
                    token_weth_pool_min_weth_threshold,
                    middleware.clone(),
                )
                .await?;

                token_weth_prices
                    .lock()
                    .unwrap()
                    .insert(self.token_b, price);

                price
            }
        };

        //Get weth value of token a in pool
        let token_b_weth_value_in_pool = BigFloat::from(reserve_1).to_f64()
            / 10f64.powf(self.token_b_decimals.into())
            / token_b_price_per_weth;

        //Return weth value in pool
        Ok(token_a_weth_value_in_pool + token_b_weth_value_in_pool)
    }
}
