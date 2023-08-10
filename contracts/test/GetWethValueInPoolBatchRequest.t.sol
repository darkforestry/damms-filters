// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./test.sol";
import "../GetWethValueInPoolBatchRequest.sol";

contract GasTest is DSTest {
    function setUp() public {}
        
    /// @dev This requires an arbitrum rpc endpoint to test.
    function testBatchContract() public {
        address[] memory pools = new address[](1);
        // pools[0] = 0xf9E9526E55a0e1Fac1813B2fE88bc9B30Eea04F9;
        // pools[0] = 0xA374094527e1673A86dE625aa59517c5dE346d32;
        pools[0] = 0x67425EE6EaC0E3DE4f560f221E9F9986dcD16037;

        address[] memory dexes = new address[](1);
        // dexes[0] = 0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32;
        dexes[0] = 0x45e5F26451CDB01B0fA1f8582E0aAD9A6F27C218;
        // dexes[0] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        uint8[] memory dexVariant = new uint8[](1);
        dexVariant[0] = 2;
        // dexIsUniV3[1] = false;
        // dexIsUniV3[0] = true;
        address weth = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
        uint256 wethInPoolThreshold = 100000000000000000000;

        GetWethValueInPoolBatchRequest batchContract = new GetWethValueInPoolBatchRequest(
                pools,
                dexes,
                dexVariant,
                weth,
                wethInPoolThreshold
            );

        // console.logBytes(address(batchContract).code);
    }
}
