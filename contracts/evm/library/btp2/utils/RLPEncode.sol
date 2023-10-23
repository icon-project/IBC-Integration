// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;

/**
 * @title RLPEncode
 * @dev A simple RLP encoding library.
 * @author Bakaoh
 * The original code was modified. For more info, please check the link:
 * https://github.com/bakaoh/solidity-rlp-encode.git
 */
library RLPEncode {
    bytes internal constant NULL = hex"f800";

    /*
     * Internal functions
     */

    /**
     * @dev RLP encodes a byte string.
     * @param self The byte string to encode.
     * @return The RLP encoded string in bytes.
     */
    function encodeBytes(bytes memory self)
        internal
        pure
        returns (bytes memory)
    {
        bytes memory encoded;
        if (self.length == 1 && uint8(self[0]) < 128) {
            encoded = self;
        } else {
            encoded = abi.encodePacked(encodeLength(self.length, 128), self);
        }
        return encoded;
    }

    /**
     * @dev RLP encodes a list of RLP encoded byte byte strings.
     * @param self The list of RLP encoded byte strings.
     * @return The RLP encoded list of items in bytes.
     */
    function encodeList(bytes[] memory self)
        internal
        pure
        returns (bytes memory)
    {
        bytes memory list = flatten(self);
        return abi.encodePacked(encodeLength(list.length, 192), list);
    }

    /**
     * @dev RLP encodes a list
     * @param self concatenated bytes of RLP encoded bytes.
     * @return The RLP encoded list of items in bytes
     */
    function encodeList(bytes memory self)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(encodeLength(self.length, 192), self);
    }

    /**
     * @dev RLP encodes a string.
     * @param self The string to encode.
     * @return The RLP encoded string in bytes.
     */
    function encodeString(string memory self)
        internal
        pure
        returns (bytes memory)
    {
        return encodeBytes(bytes(self));
    }

    /**
     * @dev RLP encodes an address.
     * @param self The address to encode.
     * @return The RLP encoded address in bytes.
     */
    function encodeAddress(address self) internal pure returns (bytes memory) {
        bytes memory inputBytes;
        assembly {
            let m := mload(0x40)
            mstore(
                add(m, 20),
                xor(0x140000000000000000000000000000000000000000, self)
            )
            mstore(0x40, add(m, 52))
            inputBytes := m
        }
        return encodeBytes(inputBytes);
    }

    /**
     * @dev RLP encodes a uint.
     * @param self The uint to encode.
     * @return The RLP encoded uint in bytes.
     */
    function encodeUint(uint256 self) internal pure returns (bytes memory) {
        return encodeBytes(uintToBytes(self));
    }

    /**
     * @dev RLP encodes an int.
     * @param self The int to encode.
     * @return The RLP encoded int in bytes.
     */
    function encodeInt(int256 self) internal pure returns (bytes memory) {
        return encodeBytes(intToBytes(self));
    }

    /**
     * @dev RLP encodes a bool.
     * @param self The bool to encode.
     * @return The RLP encoded bool in bytes.
     */
    function encodeBool(bool self) internal pure returns (bytes memory) {
        bytes memory encoded = new bytes(1);
        encoded[0] = (self ? bytes1(0x01) : bytes1(0x00));
        return encoded;
    }

    /**
     * @dev RLP encodes null.
     * @return bytes for null
     */
    function encodeNull() internal pure returns (bytes memory) {
        return NULL;
    }

    /*
     * Private functions
     */

    /**
     * @dev Encode the first byte, followed by the `len` in binary form if `length` is more than 55.
     * @param len The length of the string or the payload.
     * @param offset 128 if item is string, 192 if item is list.
     * @return RLP encoded bytes.
     */
    function encodeLength(uint256 len, uint256 offset)
        private
        pure
        returns (bytes memory)
    {
        bytes memory encoded;
        if (len < 56) {
            encoded = new bytes(1);
            encoded[0] = bytes32(len + offset)[31];
        } else {
            uint256 lenLen;
            uint256 i = 1;
            while (len / i != 0) {
                lenLen++;
                i *= 256;
            }

            encoded = new bytes(lenLen + 1);
            encoded[0] = bytes32(lenLen + offset + 55)[31];
            for (i = 1; i <= lenLen; i++) {
                encoded[i] = bytes32((len / (256**(lenLen - i))) % 256)[31];
            }
        }
        return encoded;
    }

    function lastBytesOf(int256 value, uint256 size) internal pure returns  (bytes memory){
        bytes memory buffer = new bytes(size);
        assembly {
            let dst := add(buffer, 32)
            for { let idx := sub(32,size) } lt(idx,32) { idx := add(idx, 1) } {
                mstore8(dst, byte(idx, value))
                dst := add(dst, 1)
            }
        }
        return buffer;
    }

    function intToBytes(int256 x) internal pure returns (bytes memory) {
        if (x == 0) {
            return new bytes(1);
        }
        int256 right = 0x80;
        int256 left = -0x81;
        for (uint i = 1 ; i<32 ; i++) {
            if ((x<right) && (x>left)) {
                return lastBytesOf(x, i);
            }
            right <<= 8;
            left <<= 8;
        }
        return abi.encodePacked(x);
    }

    function uintToBytes(uint256 x) internal pure returns (bytes memory) {
        if (x == 0) {
            return new bytes(1);
        }
        uint256 right = 0x80;
        for (uint i = 1 ; i<32 ; i++) {
            if (x<right) {
                return lastBytesOf(int256(x), i);
            }
            right <<= 8;
        }
        if (x<right) {
            return abi.encodePacked(x);
        } else {
            return abi.encodePacked(bytes1(0), x);
        }
    }

    /**
     * @dev Flattens a list of byte strings into one byte string.
     * @notice From: https://github.com/sammayo/solidity-rlp-encoder/blob/master/RLPEncode.sol.
     * @param _list List of byte strings to flatten.
     * @return The flattened byte string.
     */
    function flatten(bytes[] memory _list) private pure returns (bytes memory) {
        if (_list.length == 0) {
            return new bytes(0);
        }

        uint256 len;
        uint256 i;
        for (i = 0; i < _list.length; i++) {
            len += _list[i].length;
        }

        bytes memory flattened = new bytes(len);
        uint256 flattenedPtr;
        assembly {
            flattenedPtr := add(flattened, 0x20)
        }

        for (i = 0; i < _list.length; i++) {
            bytes memory item = _list[i];

            uint256 listPtr;
            assembly {
                listPtr := add(item, 0x20)
            }

            memcpy(flattenedPtr, listPtr, item.length);
            flattenedPtr += _list[i].length;
        }

        return flattened;
    }

    /**
     * @dev Copies a piece of memory to another location.
     * @notice From: https://github.com/Arachnid/solidity-stringutils/blob/master/src/strings.sol.
     * @param dest Destination location.
     * @param src Source location.
     * @param len Length of memory to copy.
     */
    function memcpy(uint dest, uint src, uint len) private pure {
        // Copy word-length chunks while possible
        for(; len >= 32; len -= 32) {
            assembly {
                mstore(dest, mload(src))
            }
            dest += 32;
            src += 32;
        }

        // Copy remaining bytes
        uint mask = type(uint).max;
        if (len > 0) {
            mask = 256 ** (32 - len) - 1;
            assembly {
                let srcpart := and(mload(src), not(mask))
                let destpart := and(mload(dest), mask)
                mstore(dest, or(destpart, srcpart))
            }
        }
    }
}
