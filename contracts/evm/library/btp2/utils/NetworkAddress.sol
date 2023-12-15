// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;

/**
   NetworkAdress 'NETWORK_ID/ACCOUNT_ADDRESS'
*/
library NetworkAddress {
    string internal constant REVERT = "invalidNetworkAddress";
    bytes internal constant DELIMITER = bytes("/");

    /**
       @notice Parse NetworkAddress address
       @param _str (String) Network Address
       @return (String) network id
       @return (String) account address
    */
    function parseNetworkAddress(
        string memory _str
    ) internal pure returns (
        string memory,
        string memory
    ) {
        uint256 offset = _validate(_str);
        return (_slice(_str, 0, offset),
        _slice(_str, offset+1, bytes(_str).length));
    }


    /**
       @notice Gets network id of Network address
       @param _str (String) Network address
       @return (String) network id
    */
    function nid(
        string memory _str
    ) internal pure returns (
        string memory
    ) {
        return _slice(_str, 0, _validate(_str));
    }

    function _validate(
        string memory _str
    ) private pure returns (
        uint256 offset
    ){
        bytes memory _bytes = bytes(_str);

        uint256 i = 0;
        for (; i < _bytes.length; i++) {
            if (_bytes[i] == DELIMITER[0]) {
                return i;
            }
        }
        revert(REVERT);
    }

    function _slice(
        string memory _str,
        uint256 _from,
        uint256 _to
    ) private pure returns (
        string memory
    ) {
        //If _str is calldata, could use slice
        //        return string(bytes(_str)[_from:_to]);
        bytes memory _bytes = bytes(_str);
        bytes memory _ret = new bytes(_to - _from);
        uint256 j = _from;
        for (uint256 i = 0; i < _ret.length; i++) {
            _ret[i] = _bytes[j++];
        }
        return string(_ret);
    }

    /**
       @notice Create Network address by network id and account address
       @param _net (String) network id
       @param _addr (String) account address
       @return (String) Network address
    */
    function networkAddress(
        string memory _net,
        string memory _addr
    ) internal pure returns (
        string memory
    ) {
        return string(abi.encodePacked(_net, DELIMITER, _addr));
    }
}
