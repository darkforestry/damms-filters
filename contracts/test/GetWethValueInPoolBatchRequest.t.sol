// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./test.sol";
import "../GetWethValueInPoolBatchRequest.sol";
import "../test/utils/Console.sol";
contract GasTest is DSTest {
    
    function setUp() public {}

    function testBatchContract() public {
        address[] memory pools = new address[](2);
        // pools[0] = 0xf9E9526E55a0e1Fac1813B2fE88bc9B30Eea04F9;
        // pools[0] = 0xA374094527e1673A86dE625aa59517c5dE346d32;
        pools[0] = 0xC31E54c7a869B9FcBEcc14363CF510d1c41fa443;
        pools[1] = 0xCd3acbFe3e86266440c424B5b2C47D758d180d67;
        address[] memory dexes = new address[](2);
        // dexes[0] = 0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32;
        dexes[0] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        dexes[1] =0x45e5F26451CDB01B0fA1f8582E0aAD9A6F27C218;
        // dexes[0] = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        uint8[] memory dexIsUniV3 = new uint8[](2);
        dexIsUniV3[0] = 1;
        dexIsUniV3[1] =2;
        // dexIsUniV3[1] = false;
        // dexIsUniV3[0] = true;
        address weth = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
        uint256 wethInPoolThreshold = 1000000000000000000;

        GetWethValueInPoolBatchRequest batchContract = new GetWethValueInPoolBatchRequest(
                pools,
                dexes,
                dexIsUniV3,
                weth,
                wethInPoolThreshold
            );

        console.logBytes(address(batchContract).code);
        uint256[] memory wethValueInPools=abi.decode(address(batchContract).code, (uint256[]));
        for (uint i=0; i< wethValueInPools.length; i++){
            console.logUint(wethValueInPools[i]);
        }
    }
}
