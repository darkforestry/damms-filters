// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "./Test.sol";
import "./Console.sol";
import "../GetWethValueInPoolBatchRequest.sol";

contract GasTest is DSTest {
    function setUp() public {}

    function testBatchContract() public {
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
