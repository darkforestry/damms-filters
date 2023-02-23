// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./Test.sol";
import "./Console.sol";
import "../GetWethValueInPoolBatchRequest.sol";

contract GasTest is DSTest {
    function setUp() public {}

    function testBatchContract() public {
        address[] memory pools = new address[](1);
        pools[0] = 0xB0e10B81f0e420eF7E3938ae224Cf2D29E3EE80f;
        // pools[0] = 0xA374094527e1673A86dE625aa59517c5dE346d32;
        address[] memory dexes = new address[](1);
        // dexes[0] = 0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32;
        // dexes[1] = 0xc35DADB65012eC5796536bD9864eD8773aBc74C4;
        dexes[0] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        bool[] memory dexIsUniV3 = new bool[](1);
        // dexIsUniV3[0] = false;
        // dexIsUniV3[1] = false;
        dexIsUniV3[0] = true;
        address weth = 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270;
        uint256 wethInPoolThreshold = 10 ether;
        GetWethValueInPoolBatchRequest batchContract = new GetWethValueInPoolBatchRequest(
                pools,
                dexes,
                dexIsUniV3,
                weth,
                wethInPoolThreshold
            );
        // console.logBytes(address(batchContract).code);
    }
}
