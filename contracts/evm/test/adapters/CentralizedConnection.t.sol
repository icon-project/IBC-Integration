// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {LZEndpointMock} from "@lz-contracts/mocks/LZEndpointMock.sol";
import "@xcall/contracts/adapters/CentralizedConnection.sol";
import "@xcall/contracts/xcall/CallService.sol";
import "@xcall/contracts/mocks/multi-protocol-dapp/MultiProtocolSampleDapp.sol";
import "@xcall/utils/Types.sol";

contract CentralizedConnectionTest is Test {
    using RLPEncodeStruct for Types.CSMessage;
    using RLPEncodeStruct for Types.CSMessageRequest;
    using RLPEncodeStruct for Types.CSMessageResponse;

    event CallExecuted(uint256 indexed _reqId, int _code, string _msg);

    event RollbackExecuted(uint256 indexed _sn);

    event Message(string targetNetwork, int256 sn, bytes msg);

    event ResponseOnHold(uint256 indexed _sn);

    MultiProtocolSampleDapp dappSource;
    MultiProtocolSampleDapp dappTarget;

    CallService xCallSource;
    CallService xCallTarget;

    CentralizedConnection adapterSource;
    CentralizedConnection adapterTarget;

    address public sourceRelayer;
    address public destinationRelayer;

    string public nidSource = "nid.source";
    string public nidTarget = "nid.target";

    address public owner = address(uint160(uint256(keccak256("owner"))));
    address public admin = address(uint160(uint256(keccak256("admin"))));
    address public user = address(uint160(uint256(keccak256("user"))));

    address public source_relayer =
        address(uint160(uint256(keccak256("source_relayer"))));
    address public destination_relayer =
        address(uint160(uint256(keccak256("destination_relayer"))));

    function _setupSource() internal {
        console2.log("------>setting up source<-------");
        xCallSource = new CallService();
        xCallSource.initialize(nidSource);

        dappSource = new MultiProtocolSampleDapp();
        dappSource.initialize(address(xCallSource));

        adapterSource = new CentralizedConnection();
        adapterSource.initialize(source_relayer, address(xCallSource));

        xCallSource.setDefaultConnection(nidTarget, address(adapterSource));

        console2.log(ParseAddress.toString(address(xCallSource)));
        console2.log(ParseAddress.toString(address(user)));
    }

    function _setupTarget() internal {
        console2.log("------>setting up target<-------");

        xCallTarget = new CallService();
        xCallTarget.initialize(nidTarget);

        dappTarget = new MultiProtocolSampleDapp();
        dappTarget.initialize(address(xCallTarget));

        adapterTarget = new CentralizedConnection();
        adapterTarget.initialize(destination_relayer, address(xCallTarget));

        xCallTarget.setDefaultConnection(nidSource, address(adapterTarget));
    }

    /**
     * @dev Sets up the initial state for the test.
     */
    function setUp() public {
        vm.startPrank(owner);

        _setupSource();
        _setupTarget();

        vm.stopPrank();

        // deal some gas
        vm.deal(admin, 10 ether);
        vm.deal(user, 10 ether);
    }

    function testSetAdmin() public {
        vm.prank(source_relayer);
        adapterSource.setAdmin(user);
        assertEq(adapterSource.admin(), user);
    }

    function testSetAdminUnauthorized() public {
        vm.prank(user);
        vm.expectRevert("OnlyRelayer");
        adapterSource.setAdmin(user);
    }

    function testSendMessage() public {
        vm.startPrank(user);
        string memory to = NetworkAddress.networkAddress(
            nidTarget,
            ParseAddress.toString(address(dappTarget))
        );

        uint256 cost = adapterSource.getFee(nidTarget, false);

        bytes memory data = bytes("test");
        bytes memory rollback = bytes("");

        dappSource.sendMessage{value: cost}(to, data, rollback);
        vm.stopPrank();
    }

    function testRecvMessage() public {
        bytes memory data = bytes("test");
        string memory iconDapp = NetworkAddress.networkAddress(
            nidSource,
            "0xa"
        );
        Types.CSMessageRequest memory request = Types.CSMessageRequest(
            iconDapp,
            ParseAddress.toString(address(dappSource)),
            1,
            false,
            data,
            new string[](0)
        );
        Types.CSMessage memory message = Types.CSMessage(
            Types.CS_REQUEST,
            request.encodeCSMessageRequest()
        );

        vm.startPrank(destination_relayer);
        adapterTarget.recvMessage(
            nidSource,
            1,
            RLPEncodeStruct.encodeCSMessage(message)
        );
        vm.stopPrank();
    }

    function testRecvMessageUnAuthorized() public {
        bytes memory data = bytes("test");
        string memory iconDapp = NetworkAddress.networkAddress(
            nidSource,
            "0xa"
        );
        Types.CSMessageRequest memory request = Types.CSMessageRequest(
            iconDapp,
            ParseAddress.toString(address(dappSource)),
            1,
            false,
            data,
            new string[](0)
        );
        Types.CSMessage memory message = Types.CSMessage(
            Types.CS_REQUEST,
            request.encodeCSMessageRequest()
        );

        vm.startPrank(user);
        vm.expectRevert("OnlyRelayer");
        adapterTarget.recvMessage(
            nidSource,
            1,
            RLPEncodeStruct.encodeCSMessage(message)
        );
        vm.stopPrank();
    }

    function testRecvMessageDuplicateMsg() public {
        bytes memory data = bytes("test");
        string memory iconDapp = NetworkAddress.networkAddress(
            nidSource,
            "0xa"
        );
        Types.CSMessageRequest memory request = Types.CSMessageRequest(
            iconDapp,
            ParseAddress.toString(address(dappSource)),
            1,
            false,
            data,
            new string[](0)
        );
        Types.CSMessage memory message = Types.CSMessage(
            Types.CS_REQUEST,
            request.encodeCSMessageRequest()
        );

        vm.startPrank(destination_relayer);
        adapterTarget.recvMessage(
            nidSource,
            1,
            RLPEncodeStruct.encodeCSMessage(message)
        );

        vm.expectRevert("Duplicate Message");
        adapterTarget.recvMessage(
            nidSource,
            1,
            RLPEncodeStruct.encodeCSMessage(message)
        );
        vm.stopPrank();
    }

    function testRevertMessage() public {
        vm.startPrank(destination_relayer);
        adapterTarget.revertMessage(1);
        vm.stopPrank();
    }

    function testRevertMessageUnauthorized() public {
        vm.startPrank(user);
        vm.expectRevert("OnlyRelayer");
        adapterTarget.revertMessage(1);
        vm.stopPrank();
    }

    function testSetFees() public {
        vm.prank(source_relayer);
        adapterSource.setFee(nidTarget, 5 ether, 5 ether);

        assertEq(adapterSource.getFee(nidTarget, true), 10 ether);
        assertEq(adapterSource.getFee(nidTarget, false), 5 ether);
    }

    function testSetFeesUnauthorized() public {
        vm.prank(user);

        vm.expectRevert("OnlyRelayer");
        adapterSource.setFee(nidTarget, 5 ether, 5 ether);
    }

    function testClaimFeesUnauthorized() public {
        vm.prank(user);

        vm.expectRevert("OnlyRelayer");
        adapterSource.claimFees();
    }

    function testClaimFees() public {
        testSetFees();
        vm.startPrank(user);
        string memory to = NetworkAddress.networkAddress(
            nidTarget,
            ParseAddress.toString(address(dappTarget))
        );

        uint256 cost = adapterSource.getFee(nidTarget, true);

        bytes memory data = bytes("test");
        bytes memory rollback = bytes("rollback");

        dappSource.sendMessage{value: cost}(to, data, rollback);
        vm.stopPrank();

        assert(address(adapterSource).balance == 10 ether);

        vm.startPrank(source_relayer);
        adapterSource.claimFees();
        vm.stopPrank();

        assert(source_relayer.balance == 10 ether);
    }

    function testGetReceipt() public {
        bytes memory data = bytes("test");
        string memory iconDapp = NetworkAddress.networkAddress(
            nidSource,
            "0xa"
        );
        Types.CSMessageRequest memory request = Types.CSMessageRequest(
            iconDapp,
            ParseAddress.toString(address(dappSource)),
            1,
            false,
            data,
            new string[](0)
        );
        Types.CSMessage memory message = Types.CSMessage(
            Types.CS_REQUEST,
            request.encodeCSMessageRequest()
        );

        assert(adapterTarget.getReceipt(nidSource, 1) == false);

        vm.startPrank(destination_relayer);
        adapterTarget.recvMessage(
            nidSource,
            1,
            RLPEncodeStruct.encodeCSMessage(message)
        );
        vm.stopPrank();

        assert(adapterTarget.getReceipt(nidSource, 1) == true);
    }
}
