// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./test.sol";
import "../GetWethValueInPoolBatchRequest.sol";

contract GasTest is DSTest {
    function setUp() public {}

    function testBatchContract() public {
        address[] memory pools = new address[](1);
        // pools[0] = 0xf9E9526E55a0e1Fac1813B2fE88bc9B30Eea04F9;
        // pools[0] = 0xA374094527e1673A86dE625aa59517c5dE346d32;
        pools[0] = 0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640;

        address[] memory dexes = new address[](1);
        // dexes[0] = 0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32;
        dexes[0] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        // dexes[0] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        bool[] memory dexIsUniV3 = new bool[](1);
        dexIsUniV3[0] = true;
        // dexIsUniV3[1] = false;
        // dexIsUniV3[0] = true;
        address weth = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
        uint256 wethInPoolThreshold = 100000000000000000000;

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
