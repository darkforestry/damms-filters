// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./test.sol";

contract GasTest is DSTest {
    function setUp() public {
        address[] memory pools = new address[](3);
        pools[0] = 0xAa1656B7d4629476Fa4CF76CCfBc01a4653bAc71;
        pools[1] = 0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc;
        pools[2] = 0xa2107FA5B38d9bbd2C461D6EDf11B11A50F6b974;
        address[] memory dexes = new address[](3);
        dexes[0] = 0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac;
        dexes[1] = 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f;
        dexes[2] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        bool[] memory dexIsUniV3 = new bool[](3);
        dexIsUniV3[0] = false;
        dexIsUniV3[1] = false;
        dexIsUniV3[2] = true;
        address weth = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
        uint256 wethInPoolThreshold = 2 ether;

        new GetWethValueInPoolBatchRequest(
            pools,
            dexes,
            dexIsUniV3,
            weth,
            wethInPoolThreshold
        );
    }
}

contract GetWethValueInPoolBatchRequest {
    uint256 internal constant Q96 = 0x1000000000000000000000000;
    address internal constant ADDRESS_ZERO = address(0);

    mapping(address => uint128) public tokenToWethPrices;

    constructor(
        address[] memory pools,
        address[] memory dexes,
        bool[] memory dexIsUniV3,
        address weth,
        uint256 wethInPoolThreshold
    ) {
        uint256[] memory wethValueInPools = new uint256[](pools.length);

        for (uint256 i = 0; i < pools.length; ++i) {
            //Get the token0 and token1 from the pool
            address token0 = IUniswapV2Pair(pools[i]).token0();
            address token1 = IUniswapV2Pair(pools[i]).token1();

            //Get the reserves from the pool
            (uint256 r0, uint256 r1) = getReserves(pools[i], token0, token1);

            //Get the value of the tokens in the pool in weth
            uint256 token0WethValueInPool = getWethValueOfTokenInPool(
                token0,
                weth,
                r0,
                dexes,
                dexIsUniV3,
                wethInPoolThreshold
            );

            uint256 token1WethValueInPool = getWethValueOfTokenInPool(
                token1,
                weth,
                r1,
                dexes,
                dexIsUniV3,
                wethInPoolThreshold
            );

            // add the aggregate weth value of both of the tokens in the pool to the wethValueInPools array
            wethValueInPools[i] = token0WethValueInPool + token1WethValueInPool;
        }

        // insure abi encoding, not needed here but increase reusability for different return types
        // note: abi.encode add a first 32 bytes word with the address of the original data
        bytes memory abiEncodedData = abi.encode(wethValueInPools);

        assembly {
            // Return from the start of the data (discarding the original data address)
            // up to the end of the memory used
            let dataStart := add(abiEncodedData, 0x20)
            return(dataStart, sub(msize(), dataStart))
        }
    }

    ///TODO: write a doc comment describing what wethInPoolThreshold is
    function getWethValueOfTokenInPool(
        address token,
        address weth,
        uint256 amount,
        address[] memory dexes,
        bool[] memory dexIsUniV3,
        uint256 wethInPoolThreshold
    ) internal returns (uint256) {
        uint128 tokenToWethPrice = tokenToWethPrices[token];

        if (tokenToWethPrice == 1) {
            return 0;
        } else {
            for (uint256 i = 0; i < dexes.length; ++i) {
                uint256 wethValueInPool = _getWethValueOfTokenInPool(
                    token,
                    weth,
                    amount,
                    tokenToWethPrice,
                    dexes[i],
                    dexIsUniV3[i],
                    wethInPoolThreshold
                );

                if (wethValueInPool != 0) {
                    return wethValueInPool;
                }
            }

            //We set the price to 1 so that we know that the token to weth pairing does not exist or is not valid
            tokenToWethPrices[token] = 1;
            return 0;
        }
    }

    function _getWethValueOfTokenInPool(
        address token,
        address weth,
        uint256 amount,
        uint128 tokenToWethPrice,
        address dexFactory,
        bool isUniV3,
        uint256 wethInPoolThreshold
    ) internal returns (uint256) {
        //If the token is weth, the amount is the amount of weth in the pool for that token
        if (token == weth) {
            return amount;
        }

        if (tokenToWethPrice > 1) {
            //Calculate the value of weth in the pool by using the amount passed in and the price that we derived
            return mul64U(tokenToWethPrice, amount);
        } else {
            // ^^ if we dont already have the price cached, that means that the price is not initialized and
            // we need to get the price from a pool from one of the dexes
            if (isUniV3) {
                uint16[3] memory feeTiers = [500, 3000, 10000];
                for (uint256 j = 0; j < feeTiers.length; ++j) {
                    address pairAddress = IUniswapV3Factory(dexFactory).getPool(
                        token < weth ? token : weth,
                        weth < token ? weth : token,
                        feeTiers[j]
                    );

                    if (pairAddress != ADDRESS_ZERO) {
                        ///Check here if the weth in pool threshold is met

                        uint256 wethValue = getTokenToWethValueV3(
                            token,
                            amount,
                            weth,
                            pairAddress,
                            wethInPoolThreshold
                        );

                        if (wethValue != 0) {
                            return wethValue;
                        }
                    }
                }
            } else {
                bool tokenIsToken0 = token < weth;

                address pairAddress = IUniswapV2Factory(dexFactory).getPair(
                    tokenIsToken0 ? token : weth,
                    tokenIsToken0 ? weth : token
                );

                if (pairAddress != ADDRESS_ZERO) {
                    uint256 wethValue = getTokenToWethValueV2(
                        token,
                        amount,
                        weth,
                        pairAddress,
                        wethInPoolThreshold
                    );

                    if (wethValue != 0) {
                        return wethValue;
                    }
                }
            }
        }

        return 0;
    }

    function getReserves(
        address lp,
        address token0,
        address token1
    ) internal returns (uint256, uint256) {
        uint256 r_x;
        uint256 r_y;

        if (lpIsNotUniV3(lp)) {
            (uint112 r_x_112, uint112 r_y_112, ) = IUniswapV2Pair(lp)
                .getReserves();
            r_x = r_x_112;
            r_y = r_y_112;
        } else {
            (r_x, r_y) = calculateV3VirtualReserves(lp);
        }

        return
            normalizeTo18Dec(
                r_x,
                r_y,
                IERC20(token0).decimals(),
                IERC20(token1).decimals()
            );
    }

    function normalizeTo18Dec(
        uint256 x,
        uint256 y,
        uint8 decimals_x,
        uint8 decimals_y
    ) internal pure returns (uint256 r_x, uint256 r_y) {
        r_x = decimals_x <= 18
            ? x**(18 - decimals_x)
            : x / (10**(decimals_x - 18));
        r_y = decimals_y <= 18
            ? y**(18 - decimals_y)
            : y / (10**(decimals_y - 18));
    }

    function getTokenToWethValueV2(
        address token,
        uint256 tokenAmount,
        address weth,
        address pool,
        uint256 wethLiquidityThreshold
    ) internal returns (uint256) {
        bool tokenIsToken0 = token < weth;

        (uint256 r_0, uint256 r_1) = getReserves(
            pool,
            tokenIsToken0 ? token : weth,
            tokenIsToken0 ? weth : token
        );

        //Check if the weth value meets the threshold
        if (tokenIsToken0) {
            if (r_0 < wethLiquidityThreshold) {
                return 0;
            }
        } else {
            if (r_1 < wethLiquidityThreshold) {
                return 0;
            }
        }

        uint128 price = divuu(
            tokenIsToken0 ? r_1 : r_0,
            tokenIsToken0 ? r_0 : r_1
        );
        //Add the price to the tokenToWeth price mapping
        tokenToWethPrices[token] = price;

        return mul64U(price, tokenAmount);
    }

    function getTokenToWethValueV3(
        address token,
        uint256 tokenAmount,
        address weth,
        address pool,
        uint256 wethLiquidityThreshold
    ) internal returns (uint256) {
        bool tokenIsToken0 = token < weth;
        (uint256 r_0, uint256 r_1) = calculateV3VirtualReserves(pool);

        if (tokenIsToken0) {
            if (r_0 < wethLiquidityThreshold) {
                return 0;
            }
        } else {
            if (r_1 < wethLiquidityThreshold) {
                return 0;
            }
        }

        uint128 price = tokenIsToken0 ? divuu(r_1, r_0) : divuu(r_0, r_1);

        //Add the price to the tokenToWeth price mapping
        tokenToWethPrices[token] = price;

        return mul64U(price, tokenAmount);
    }

    ///Does not normalize to 18 decimals
    function calculateV3VirtualReserves(address pool)
        internal
        view
        returns (uint256 r_0, uint256 r_1)
    {
        (uint160 sqrtPriceX96, , , , , , ) = IUniswapV3PoolState(pool).slot0();
        uint128 liquidity = IUniswapV3PoolState(pool).liquidity();
        unchecked {
            uint256 sqrtPriceInv = (2**192 / sqrtPriceX96);

            uint256 lo_r0 = (uint256(sqrtPriceInv) *
                (uint256(liquidity) & (2**64))) >> 96;
            uint256 hi_r0 = (uint256(sqrtPriceInv) *
                (uint256(liquidity) >> 96));
            uint256 lo_r1 = (uint256(sqrtPriceX96) *
                (uint256(liquidity) & (2**64))) >> 96;
            uint256 hi_r1 = (uint256(sqrtPriceX96) *
                (uint256(liquidity) >> 96));

            hi_r0 <<= 96;
            hi_r1 <<= 96;

            require(hi_r0 <= type(uint128).max);
            require(hi_r1 <= type(uint128).max);

            (r_0, r_1) = (hi_r0 + lo_r0, hi_r1 + lo_r1);
        }
    }

    /// @notice helper to divide two unsigned integers
    /// @param x uint256 unsigned integer
    /// @param y uint256 unsigned integer
    /// @return unsigned 64.64 fixed point number
    function divuu(uint256 x, uint256 y) internal pure returns (uint128) {
        unchecked {
            require(y != 0);

            uint256 answer;

            if (x <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
                answer = (x << 64) / y;
            else {
                uint256 msb = 192;
                uint256 xc = x >> 192;
                if (xc >= 0x100000000) {
                    xc >>= 32;
                    msb += 32;
                }
                if (xc >= 0x10000) {
                    xc >>= 16;
                    msb += 16;
                }
                if (xc >= 0x100) {
                    xc >>= 8;
                    msb += 8;
                }
                if (xc >= 0x10) {
                    xc >>= 4;
                    msb += 4;
                }
                if (xc >= 0x4) {
                    xc >>= 2;
                    msb += 2;
                }
                if (xc >= 0x2) msb += 1; // No need to shift xc anymore

                answer = (x << (255 - msb)) / (((y - 1) >> (msb - 191)) + 1);
                require(
                    answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
                    "overflow in divuu"
                );

                uint256 hi = answer * (y >> 128);
                uint256 lo = answer * (y & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);

                uint256 xh = x >> 192;
                uint256 xl = x << 64;

                if (xl < lo) xh -= 1;
                xl -= lo; // We rely on overflow behavior here
                lo = hi << 128;
                if (xl < lo) xh -= 1;
                xl -= lo; // We rely on overflow behavior here

                assert(xh == hi >> 128);

                answer += xl / y;
            }

            require(
                answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
                "overflow in divuu last"
            );
            return uint128(answer);
        }
    }

    /// @notice helper function to multiply unsigned 64.64 fixed point number by a unsigned integer
    /// @param x 64.64 unsigned fixed point number
    /// @param y uint256 unsigned integer
    /// @return unsigned
    function mul64U(uint128 x, uint256 y) internal pure returns (uint256) {
        unchecked {
            if (y == 0 || x == 0) {
                return 0;
            }

            uint256 lo = (uint256(x) *
                (y & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)) >> 64;
            uint256 hi = uint256(x) * (y >> 128);

            require(hi <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
            hi <<= 64;

            require(
                hi <=
                    0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff -
                        lo
            );
            return hi + lo;
        }
    }

    ///@notice Helper function to determine if a pool address is Uni V2 compatible.
    ///@param lp - Pair address.
    ///@return bool Indicator whether the pool is not Uni V3 compatible.
    function lpIsNotUniV3(address lp) internal returns (bool) {
        bool success;
        assembly {
            //store the function sig for  "fee()"
            mstore(
                0x00,
                0xddca3f4300000000000000000000000000000000000000000000000000000000
            )

            success := call(
                gas(), // gas remaining
                lp, // destination address
                0, // no ether
                0x00, // input buffer (starts after the first 32 bytes in the `data` array)
                0x04, // input length (loaded from the first 32 bytes in the `data` array)
                0x00, // output buffer
                0x00 // output length
            )
        }
        ///@notice return the opposite of success, meaning if the call succeeded, the address is univ3, and we should
        ///@notice indicate that lpIsNotUniV3 is false
        return !success;
    }
}

//=======================================
// Interfaces
//Note: Just flattening this to keep everything in one place for the batch contract
//=======================================

interface IUniswapV3Factory {
    function getPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) external view returns (address pool);
}

interface IERC20 {
    function decimals() external view returns (uint8);
}

interface IUniswapV2Factory {
    function getPair(address tokenA, address tokenB)
        external
        view
        returns (address pair);
}

interface IUniswapV2Pair {
    function decimals() external pure returns (uint8);

    function token0() external view returns (address);

    function token1() external view returns (address);

    function getReserves()
        external
        view
        returns (
            uint112 reserve0,
            uint112 reserve1,
            uint32 blockTimestampLast
        );
}

interface IUniswapV3PoolState {
    function slot0()
        external
        view
        returns (
            uint160 sqrtPriceX96,
            int24 tick,
            uint16 observationIndex,
            uint16 observationCardinality,
            uint16 observationCardinalityNext,
            uint8 feeProtocol,
            bool unlocked
        );

    function liquidity() external view returns (uint128);
}
