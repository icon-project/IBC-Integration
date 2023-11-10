// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;
pragma abicoder v2;

import "openzeppelin-contracts-upgradeable/contracts/proxy/utils/Initializable.sol";
import "@xcall/utils/Types.sol";
import "@iconfoundation/btp2-solidity-library/interfaces/ICallService.sol";
import "@lz-contracts/interfaces/ILayerZeroReceiver.sol";
import "@lz-contracts/interfaces/ILayerZeroEndpoint.sol";
import "./interfaces/ILayerZeroAdapter.sol";
import "@xcall/contracts/xcall/interfaces/IConnection.sol";

/**
 * @title LayerZeroAdapter
 * @dev A contract serves as a cross-chain xcall adapter, enabling communication between xcall on different blockchain networks via LayerZero.
 */
contract LayerZeroAdapter is ILayerZeroAdapter, Initializable, ILayerZeroReceiver, IConnection {
    bytes constant private EMPTY_BYTES = new bytes(2048);
    mapping(uint256 => Types.PendingResponse) private pendingResponses;
    mapping(string => uint16) private chainIds;
    mapping(uint16 => string) private networkIds;
    mapping(string => bytes) private adapterParams;

    mapping(string => uint256) private gasLimits;
    mapping(string => uint256) private responseFees;

    mapping(string => bytes) private remoteEndpoint;
    address private layerZeroEndpoint;
    address private xCall;
    address private owner;
    address private adminAddress;

    modifier onlyOwner() {
        require(msg.sender == owner, "OnlyOwner");
        _;
    }

    modifier onlyAdmin() {
        require(msg.sender == adminAddress, "OnlyAdmin");
        _;
    }

    /**
     * @dev Initializes the contract with LayerZero endpoint and xCall address.
     * @param _layerZeroEndpoint The address of the LayerZero endpoint contract.
     * @param _xCall The address of the xCall contract.
     */
    function initialize(address _layerZeroEndpoint, address _xCall) public initializer {
        owner = msg.sender;
        adminAddress = msg.sender;
        layerZeroEndpoint = _layerZeroEndpoint;
        xCall = _xCall;
    }

    /**
     * @dev Configure connection settings for a destination chain.
     * @param networkId The network ID of the destination chain.
     * @param chainId The chain ID of the destination chain.
     * @param endpoint The endpoint or address of the destination chain.
     * @param gasLimit The gas limit for the connection on the destination chain.
     * @param responseFee The fee required for a response from the destination chain, to be airdropped to the specified `endpoint`.
     */
    function configureConnection(
        string memory networkId,
        uint16 chainId,
        bytes memory endpoint,
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
     * @dev Get the gas fee required to send a message to a specified destination network.
     * @param _to The network ID of the target chain.
     * @param _response Indicates whether the response fee is included (true) or not (false).
     * @return _fee The fee for sending a message to the given destination network.
     */
    function getFee(string memory _to, bool _response) external view override returns (uint256 _fee) {
        bytes memory params;
        if (_response) {
            params = abi.encodePacked(uint16(2), gasLimits[_to], responseFees[_to], remoteEndpoint[_to]);
        } else {
            params = abi.encodePacked(uint16(1), gasLimits[_to]);
        }
        (_fee,) = ILayerZeroEndpoint(layerZeroEndpoint).estimateFees(chainIds[_to], address(this), EMPTY_BYTES, false, params);
    }

    /**
     * @dev Send a message to a specified destination network.
     * @param _to The network ID of the destination network.
     * @param _svc The name of the service.
     * @param _sn The serial number of the message.
     * @param _msg The serialized bytes of the service message.
     */
    function sendMessage(
        string memory _to,
        string memory _svc,
        int256 _sn,
        bytes memory _msg
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
        bytes memory params;
        if (_sn > 0) {
            params = abi.encodePacked(uint16(2), gasLimits[_to], responseFees[_to], remoteEndpoint[_to]);
        } else {
            params = abi.encodePacked(uint16(1), gasLimits[_to]);
        }


        ILayerZeroEndpoint(layerZeroEndpoint).send{value: fee}(
            chainIds[_to],
            abi.encodePacked(remoteEndpoint[_to], address(this)),
            abi.encodePacked(_msg),
            payable(address(this)),
            address(0x0),
            params
        );
    }

    /**
     * @dev Endpoint that the LayerZero Relayer contract calls to deliver the payload.
     * @param sourceChain The source chain ID.
     * @param _srcAddress The source address.
     * @param _nonce The nonce.
     * @param payload The payload to be delivered.
     */
    function lzReceive(
        uint16 sourceChain,
        bytes memory _srcAddress,
        uint64 _nonce,
        bytes memory payload
    ) public override {
        require(msg.sender == layerZeroEndpoint, "Invalid endpoint caller");
        string memory nid = networkIds[sourceChain];
        require(keccak256(_srcAddress) == keccak256(abi.encodePacked(remoteEndpoint[nid], address(this))), "Source address mismatched");
        ICallService(xCall).handleMessage(nid, payload);
    }

    /**
     * @dev Pay and trigger the execution of a stored response to be sent back.
     * @param _sn The serial number of the message for which the response is being triggered.
     */
    function triggerResponse(uint256 _sn) external override payable {
        int256 sn = int256(_sn);
        Types.PendingResponse memory resp = pendingResponses[_sn];
        delete pendingResponses[_sn];
        uint256 fee = msg.value;

        bytes memory params = abi.encodePacked(uint16(1), gasLimits[resp.targetNetwork]);

        ILayerZeroEndpoint(layerZeroEndpoint).send{value: fee}(
            chainIds[resp.targetNetwork],
            abi.encodePacked(remoteEndpoint[resp.targetNetwork], address(this)),
            abi.encodePacked(resp.msg),
            payable(address(this)),
            address(0x0),
            params
        );
    }

    /**
     * @dev Set the address of the admin.
     * @param _address The address of the admin.
     */
    function setAdmin(address _address) external onlyAdmin {
        adminAddress = _address;
    }

    /**
     * @dev Get the address of the admin.
     * @return (Address) The address of the admin.
     */
    function admin() external view returns (address) {
        if (adminAddress == address(0)) {
            return owner;
        }
        return adminAddress;
    }

    fallback() external payable {}
}
