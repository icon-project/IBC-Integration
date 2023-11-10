// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {LZEndpointMock} from "@lz-contracts/mocks/LZEndpointMock.sol";
import "@xcall/contracts/adapters/LayerZeroAdapter.sol";
import "@xcall/contracts/xcall/CallService.sol";
import "@xcall/contracts/mocks/multi-protocol-dapp/MultiProtocolSampleDapp.sol";


contract LayerZeroAdapterTest is Test {

    LZEndpointMock public sourceEndpoint;
    LZEndpointMock public destinationEndpoint;

    uint16 public constant SourceChainID = 0x11;
    uint16 public constant DestinationChainID = 0x22;

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

    LayerZeroAdapter adapterSource;
    LayerZeroAdapter adapterTarget;

    string public nidSource = "nid.source";
    string public nidTarget = "nid.target";

    address public owner = address(uint160(uint256(keccak256("owner"))));
    address public admin = address(uint160(uint256(keccak256("admin"))));
    address public user = address(uint160(uint256(keccak256("user"))));

    function _setupSource() internal {
        console2.log("------>setting up source<-------");
        xCallSource = new CallService();
        xCallSource.initialize(nidSource);

        dappSource = new MultiProtocolSampleDapp();
        dappSource.initialize(address(xCallSource));

        adapterSource = new LayerZeroAdapter();
        adapterSource.initialize(address(sourceEndpoint), address(xCallSource));

        xCallSource.setDefaultConnection(nidTarget, address(adapterSource));
    }

    function _setupTarget() internal {
        console2.log("------>setting up target<-------");

        xCallTarget = new CallService();
        xCallTarget.initialize(nidTarget);

        dappTarget = new MultiProtocolSampleDapp();
        dappTarget.initialize(address(xCallTarget));

        adapterTarget = new LayerZeroAdapter();
        adapterTarget.initialize(address(destinationEndpoint), address(xCallTarget));

        xCallTarget.setDefaultConnection(nidSource, address(adapterTarget));
    }

    /**
     * @dev Sets up the initial state for the test.
     */
    function setUp() public {
        vm.startPrank(owner);
        // setup mock endpoint
        sourceEndpoint = new LZEndpointMock(SourceChainID);
        destinationEndpoint = new LZEndpointMock(DestinationChainID);

        _setupSource();
        _setupTarget();

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
            DestinationChainID,
            abi.encodePacked(address(adapterTarget)),
            uint256(900_000),
            uint256(1e10)
        );

        dappTarget.addConnection(nidSource, adapterTargetAdr, adapterSourceAdr);

        adapterTarget.configureConnection(
            nidSource,
            SourceChainID,
            abi.encodePacked(address(adapterSource)),
            uint256(900_000),
            uint256(1e10)
        );

        sourceEndpoint.setDestLzEndpoint(address(adapterTarget), address(destinationEndpoint));
        destinationEndpoint.setDestLzEndpoint(address(adapterSource), address(sourceEndpoint));

        adapterSource.setAdmin(admin);
        adapterTarget.setAdmin(admin);

        vm.stopPrank();

        // deal some gas
        vm.deal(admin, 10 ether);
        vm.deal(user, 10 ether);
    }


    function testSetAdmin() public {
        vm.prank(admin);
        adapterSource.setAdmin(user);
        assertEq(adapterSource.admin(), user);
    }

    function testSetAdminUnauthorized() public {
        vm.prank(user);
        vm.expectRevert("OnlyAdmin");
        adapterSource.setAdmin(user);
    }

    function testConnection() public {
        vm.prank(user);
        vm.expectRevert("OnlyAdmin");
        adapterSource.configureConnection(
            nidTarget,
            DestinationChainID,
            abi.encodePacked(address(adapterTarget)),
            uint256(900_000),
            uint256(1e16)
        );

        vm.prank(admin);
        vm.expectRevert("Connection already configured");

        adapterSource.configureConnection(
            nidTarget,
            DestinationChainID,
            abi.encodePacked(address(adapterTarget)),
            uint256(900_000),
            uint256(1e16)
        );

    }


    function testSendMessage() public {

        vm.startPrank(user);

        console2.log(abi.encodePacked(address(adapterTarget)).length);
        string memory to = NetworkAddress.networkAddress(nidTarget, ParseAddress.toString(address(dappTarget)));

        uint256 cost = adapterSource.getFee(nidTarget, false);

        bytes memory data = bytes("test");
        bytes memory rollback = bytes("");

        dappSource.sendMessage{value: cost}(to, data, rollback);

        vm.expectEmit(address(xCallTarget));
        emit CallExecuted(1, 1, "");
        xCallTarget.executeCall(1, data);
        vm.stopPrank();
    }

    function testRollback() public {
        vm.startPrank(user);


        string memory to = NetworkAddress.networkAddress(nidTarget, ParseAddress.toString(address(dappTarget)));

        uint256 cost = adapterSource.getFee(nidTarget, true);

        bytes memory data = bytes("rollback");
        bytes memory rollback = bytes("rollback-data");
        dappSource.sendMessage{value: cost}(to, data, rollback);


        vm.expectEmit();
        emit CallExecuted(1, 0, "rollback");

        emit ResponseOnHold(1);
        xCallTarget.executeCall(1, data);

        cost = adapterTarget.getFee(nidSource, true);
        adapterTarget.triggerResponse{value: cost}(1);

        vm.expectEmit();
        emit RollbackExecuted(1);
        xCallSource.executeRollback(1);
        vm.stopPrank();
    }
}