// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "@xcall/contracts/xcall/CallService.sol";

contract CallServiceScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        string memory nid = vm.envString("BSC_NID");
        string memory iconNid = vm.envString("ICON_NID");
        address connection = vm.envAddress("BMC_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);
        CallService xcall = new CallService();
        xcall.initialize(nid);

        xcall.setProtocolFee(vm.envUint("PROTOCOL_FEE"));
        xcall.setProtocolFeeHandler(vm.envAddress("OWNER_ADDRESS"));

        xcall.setDefaultConnection(iconNid, connection);
        vm.stopBroadcast();

    }
}
