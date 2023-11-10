// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;
pragma abicoder v2;

import "openzeppelin-contracts-upgradeable/contracts/proxy/utils/Initializable.sol";
import "wormhole-solidity-sdk/interfaces/IWormholeRelayer.sol";
import "wormhole-solidity-sdk/interfaces/IWormholeReceiver.sol";
import "wormhole-solidity-sdk/Utils.sol";

import "./interfaces/IWormholeAdapter.sol";

import "@xcall/utils/Types.sol";
import "@xcall/contracts/xcall/interfaces/IConnection.sol";
import "@iconfoundation/btp2-solidity-library/interfaces/ICallService.sol";

/**
 * @title WormholeAdapter
 * @dev This contract serves as a cross-chain xcall adapter, enabling communication between xcall on different blockchain networks via Wormhole.
 */
contract WormholeAdapter is IWormholeAdapter, Initializable, IWormholeReceiver, IConnection {
    mapping(uint256 => Types.PendingResponse) private pendingResponses;
    mapping(string => uint16) private chainIds;
    mapping(uint16 => string) private networkIds;
    mapping(string => uint256) private gasLimits;
    mapping(string => uint256) private responseFees;
    mapping(string => bytes32) private remoteEndpoint;
    mapping(bytes32 => bool) public seenDeliveryVaaHashes;
    address private wormholeRelayer;
    address private xCall;
    address private owner;
    address private adminAddress;

    modifier onlyOwner() {
        require(msg.sender == owner, "OnlyOwner");
        _;
    }

    modifier onlyAdmin() {
        require(msg.sender == this.admin(), "OnlyAdmin");
        _;
    }

    function initialize(address _wormholeRelayer, address _xCall) public initializer {
        owner = msg.sender;
        adminAddress = msg.sender;
        wormholeRelayer = _wormholeRelayer;
        xCall = _xCall;
    }

    /**
     * @notice Configure connection settings for a destination chain.
     * @param networkId The network ID of the destination chain.
     * @param chainId The chain ID of the destination chain.
     * @param endpoint The endpoint or address of the destination chain.
     * @param gasLimit The gas limit for transactions on the destination chain.
     * @param responseFee The fee required for a response from the destination chain, to be airdropped to the specified `endpoint`.
     */
    function configureConnection(
        string calldata networkId,
        uint16 chainId,
        bytes32 endpoint,
        uint256 gasLimit,
        uint256 responseFee
    ) external override onlyAdmin {
        require(bytes(networkIds[chainId]).length == 0, "Connection already configured");
        networkIds[chainId] = networkId;
        chainIds[networkId] = chainId;
        remoteEndpoint[networkId] = endpoint;
        gasLimits[networkId] = gasLimit;
        responseFees[networkId] = responseFee;
    }

    /**
* @notice set or update gas limit for a destination chain.
     * @param networkId The network ID of the destination chain.
     * @param gasLimit The gas limit for transactions on the destination chain.
     */
    function setGasLimit(
        string calldata networkId,
        uint256 gasLimit
    ) external override onlyAdmin {
        gasLimits[networkId] = gasLimit;
    }

    /**
* @notice set or update response fee to a source chain.
     * @param networkId The network ID of the destination chain.
     * @param responseFee The fee required for a response from the destination chain, to be airdropped to the specified `endpoint`.
     */
    function setResponseFee(
        string calldata networkId,
        uint256 responseFee
    ) external override onlyAdmin {
        responseFees[networkId] = responseFee;
    }

    /**
     * @notice Get the gas fee required to send a message to a specified destination network.
     * @param _to The network ID of the target chain.
     * @param _response Indicates whether the response fee is included (true) or not (false).
     * @return _fee The fee for sending a message to the given destination network.
     */
    function getFee(string memory _to, bool _response) external view override returns (uint256 _fee) {
        uint256 gasLimit = gasLimits[_to];
        uint256 responseFee = 0;
        if (_response) {
            responseFee = responseFees[_to];
        }
        (_fee,) = IWormholeRelayer(wormholeRelayer).quoteEVMDeliveryPrice(chainIds[_to], responseFee, gasLimit);
    }

    /**
     * @notice Send a message to a specified destination network.
     * @param _to The network ID of the destination network.
     * @param _svc The name of the service.
     * @param _sn The serial number of the message.
     * @param _msg The serialized bytes of the service message.
     */
    function sendMessage(
        string memory _to,
        string memory _svc,
        int256 _sn,
        bytes calldata _msg
    ) external override payable {
        require(msg.sender == xCall, "Only xCall can send messages");
        uint256 fee = msg.value;
        if (_sn < 0) {
            fee = this.getFee(_to, false);
            if (address(this).balance < fee) {
                uint256 sn = uint256(- _sn);
                pendingResponses[sn] = Types.PendingResponse(_msg, _to);
                emit ResponseOnHold(sn);
                return;
            }
        }

        IWormholeRelayer(wormholeRelayer).sendPayloadToEvm{value: fee}(
            chainIds[_to],
            fromWormholeFormat(remoteEndpoint[_to]),
            abi.encodePacked(_msg),
            _sn > 0 ? responseFees[_to] : 0,
            gasLimits[_to]
        );
    }

    /**
     * @notice Endpoint that the Wormhole Relayer contract calls to deliver the payload.
     */
    function receiveWormholeMessages(
        bytes memory payload,
        bytes[] memory, // additionalVaas
        bytes32 sourceAddress,
        uint16 sourceChain,
        bytes32 deliveryHash
    ) public payable override {
        require(msg.sender == wormholeRelayer, "Only relayer allowed");
        require(!seenDeliveryVaaHashes[deliveryHash], "Message already processed");
        seenDeliveryVaaHashes[deliveryHash] = true;
        string memory nid = networkIds[sourceChain];
        require(keccak256(abi.encodePacked(sourceAddress)) == keccak256(abi.encodePacked(remoteEndpoint[nid])), "source address mismatched");
        ICallService(xCall).handleMessage(nid, payload);
    }

    /**
     * @notice Pay and trigger the execution of a stored response to be sent back.
     * @param _sn The serial number of the message for which the response is being triggered.
     */
    function triggerResponse(uint256 _sn) external override payable {
        int256 sn = int256(_sn);
        Types.PendingResponse memory resp = pendingResponses[_sn];
        delete pendingResponses[_sn];
        uint256 fee = msg.value;
        IWormholeRelayer(wormholeRelayer).sendPayloadToEvm{value: fee}(
            chainIds[resp.targetNetwork],
            fromWormholeFormat(remoteEndpoint[resp.targetNetwork]),
            abi.encodePacked(resp.msg),
            0,
            gasLimits[resp.targetNetwork]
        );
    }

    /**
     * @notice Set the address of the admin.
     * @param _address The address of the admin.
     */
    function setAdmin(address _address) external onlyAdmin {
        adminAddress = _address;
    }

    /**
    * @notice Gets the address of admin
    * @return (Address) the address of admin
    */
    function admin(
    ) external view returns (
        address
    ) {
        if (adminAddress == address(0)) {
            return owner;
        }
        return adminAddress;
    }
}
