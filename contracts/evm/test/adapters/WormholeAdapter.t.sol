// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "wormhole-solidity-sdk/testing/WormholeRelayerTest.sol";
import "@xcall/contracts/adapters/WormholeAdapter.sol";
import "@xcall/contracts/xcall/CallService.sol";
import "@xcall/contracts/mocks/multi-protocol-dapp/MultiProtocolSampleDapp.sol";

contract WormholeAdapterTest is WormholeRelayerBasicTest {

    event CallExecuted(
        uint256 indexed _reqId,
        int _code,
        string _msg
    );

    event RollbackExecuted(
        uint256 indexed _sn
    );

    event ResponseOnHold(uint256 indexed _sn);

    MultiProtocolSampleDapp dappSource;
    MultiProtocolSampleDapp dappTarget;

    CallService xCallSource;
    CallService xCallTarget;

    WormholeAdapter adapterSource;
    WormholeAdapter adapterTarget;

    string public nidSource = "nid.source";
    string public nidTarget = "nid.target";


    address public admin = address(0x1111);
    address public user = address(0x1234);


    function setUpSource() public override {
        console2.log("------>setting up source<-------");
        xCallSource = new CallService();
        xCallSource.initialize(nidSource);

        dappSource = new MultiProtocolSampleDapp();
        dappSource.initialize(address(xCallSource));

        adapterSource = new WormholeAdapter();
        adapterSource.initialize(address(relayerSource), address(xCallSource));

        xCallSource.setDefaultConnection(nidTarget, address(adapterSource));
    }

    function setUpTarget() public override {
        console2.log("------>setting up target<-------");

        xCallTarget = new CallService();
        xCallTarget.initialize(nidTarget);

        dappTarget = new MultiProtocolSampleDapp();
        dappTarget.initialize(address(xCallTarget));

        adapterTarget = new WormholeAdapter();
        adapterTarget.initialize(address(relayerTarget), address(xCallTarget));

        xCallTarget.setDefaultConnection(nidSource, address(adapterTarget));
        toWormholeFormat(address(xCallTarget));

    }

    function setUpGeneral() public override {
        console2.log("------>setting up connections<-------");

        string memory adapterSourceAdr = ParseAddress.toString(
            address(adapterSource)
        );
        string memory adapterTargetAdr = ParseAddress.toString(
            address(adapterTarget)
        );


        dappSource.addConnection(nidTarget, adapterSourceAdr, adapterTargetAdr);

        adapterSource.configureConnection(
            nidTarget,
            targetChain,
            toWormholeFormat(address(adapterTarget)),
            5_000_000,
            uint256(1e14)
        );
        vm.selectFork(targetFork);
        dappTarget.addConnection(nidSource, adapterTargetAdr, adapterSourceAdr);

        adapterTarget.configureConnection(
            nidSource,
            sourceChain,
            toWormholeFormat(address(adapterSource)),
            5_000_000,
            uint256(1e14)
        );
    }

    function testSetAdmin() public {
        adapterSource.setAdmin(admin);
        assertEq(adapterSource.admin(), admin);
    }

    function testSetAdminUnauthorized() public {
        vm.prank(user);
        vm.expectRevert("OnlyAdmin");
        adapterSource.setAdmin(user);
    }

    function testConnection() public {
        vm.selectFork(sourceFork);
        adapterSource.setAdmin(admin);
        vm.prank(user);
        vm.expectRevert("OnlyAdmin");
        adapterSource.configureConnection(
            nidTarget,
            targetChain,
            toWormholeFormat(address(adapterTarget)),
            5_000_000,
            uint256(1e14)
        );

        vm.prank(admin);
        vm.expectRevert("Connection already configured");

        adapterSource.configureConnection(
            nidTarget,
            targetChain,
            toWormholeFormat(address(adapterTarget)),
            5_000_000,
            uint256(1e14)
        );

    }


    function testSendMessage() public {
        vm.recordLogs();
        vm.selectFork(sourceFork);

        string memory to = NetworkAddress.networkAddress(nidTarget, ParseAddress.toString(address(dappTarget)));

        uint256 cost = adapterSource.getFee(nidTarget, true);

        bytes memory data = bytes("test");
        bytes memory rollback = bytes("");
        dappSource.sendMessage{value: cost}(to, data, rollback);

        performDelivery();

        vm.selectFork(targetFork);
        vm.expectEmit();
        emit CallExecuted(1, 1, "");
        xCallTarget.executeCall(1, data);
    }

    function testRollback() public {
        vm.recordLogs();
        vm.selectFork(sourceFork);

        string memory to = NetworkAddress.networkAddress(nidTarget, ParseAddress.toString(address(dappTarget)));

        uint256 cost = adapterSource.getFee(nidTarget, true);

        bytes memory data = bytes("rollback");
        bytes memory rollback = bytes("rollback-data");
        dappSource.sendMessage{value: cost}(to, data, rollback);

        performDelivery();

        vm.selectFork(targetFork);
        vm.expectEmit();
        emit CallExecuted(1, 0, "rollback");

        emit ResponseOnHold(1);
        xCallTarget.executeCall(1, data);


        // trigger response
        cost = adapterTarget.getFee(nidSource, false);
        adapterTarget.triggerResponse{value: cost}(1);
        performDelivery();

        //execute rollback
        vm.selectFork(sourceFork);
        vm.expectEmit();
        emit RollbackExecuted(1);
        xCallSource.executeRollback(1);
    }


}
