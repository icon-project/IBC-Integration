// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;
pragma abicoder v2;

interface IConnection {

    /**
        @notice Send the message to a specific network.
        @dev Caller must be an registered BSH.
        @param _to      Network id of destination network
        @param _svc     Name of the service
        @param _sn      Serial number of the message
        @param _msg     Serialized bytes of Service Message
     */
    function sendMessage(
        string memory _to,
        string memory _svc,
        int256 _sn,
        bytes memory _msg
    ) external payable;

    /**
       @notice Gets the fee to the target network
       @dev _response should be true if it uses positive value for _sn of {@link #sendMessage}.
            If _to is not reachable, then it reverts.
            If _to does not exist in the fee table, then it returns zero.
       @param  _to       String ( Network ID of destionation chain )
       @param  _response Boolean ( Whether the responding fee is included )
       @return _fee      Integer (The fee of sending a message to a given destination network )
     */
    function getFee(
        string memory _to,
        bool _response
    ) external view returns (
        uint256 _fee
    );

    /**
     * @dev Set the address of the admin.
     * @param _address The address of the admin.
     */
    function setAdmin(address _address) external;

    /**
     * @dev Get the address of the admin.
     * @return (Address) The address of the admin.
     */
    function admin() external view returns (address);
}
