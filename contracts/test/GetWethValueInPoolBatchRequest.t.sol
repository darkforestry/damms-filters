// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./Test.sol";
import "./Console.sol";
import "../GetWethValueInPoolBatchRequest.sol";

contract GasTest is DSTest {
    function setUp() public {}

    function testBatchContract() public {
        address[] memory pools = new address[](1);
        pools[0] = 0x55a4CdFaE42d5922f97d6Eaa56B6c17623c9f386;
        address[] memory dexes = new address[](3);
        dexes[0] = 0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32;
        dexes[1] = 0xc35DADB65012eC5796536bD9864eD8773aBc74C4;
        dexes[2] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        bool[] memory dexIsUniV3 = new bool[](3);
        dexIsUniV3[0] = false;
        dexIsUniV3[1] = false;
        dexIsUniV3[2] = true;
        address weth = 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270;
        uint256 wethInPoolThreshold = 2 ether;

        GetWethValueInPoolBatchRequest batchContract = new GetWethValueInPoolBatchRequest(
                pools,
                dexes,
                dexIsUniV3,
                weth,
                wethInPoolThreshold
            );

        console.logBytes(address(batchContract).code);
    }
}
