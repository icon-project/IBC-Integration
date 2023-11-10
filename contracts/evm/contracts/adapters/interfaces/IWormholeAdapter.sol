// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;

/**
 * @title IWormholeAdapter.sol - Interface for Wormhole-xCall Adapter
 * @dev This interface defines the functions and events for a Wormhole-xCall adapter,
 * allowing communication and message transfer between xCall on different blockchain networks.
 */
interface IWormholeAdapter {
    /**
     * @notice Emitted when a response is put on hold.
     * @param _sn The serial number of the response.
     */
    event ResponseOnHold(uint256 indexed _sn);

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
    ) external;

    /**
 * @notice set or update gas limit for a destination chain.
     * @param networkId The network ID of the destination chain.
     * @param gasLimit The gas limit for transactions on the destination chain.
     */
    function setGasLimit    (
        string calldata networkId,
        uint256 gasLimit
    ) external;

    /**
* @notice set or update response fee to a source chain.
     * @param networkId The network ID of the destination chain.
     * @param responseFee The response fee for transactions from the destination chain.
     * @param responseFee The fee required for a response from the destination chain, to be airdropped to the specified `endpoint`.
     */
    function setResponseFee    (
        string calldata networkId,
        uint256 responseFee
    ) external;

    /**
     * @notice Pay and trigger the execution of a stored response to be sent back.
     * @param _sn The serial number of the message for which the response is being triggered.
     */
    function triggerResponse(uint256 _sn) external payable;
}
