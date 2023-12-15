// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;
pragma abicoder v2;

import "./interfaces/IFeeManage.sol";
import "./interfaces/IConnection.sol";
import "@xcall/utils/RLPDecodeStruct.sol";
import "@xcall/utils/RLPEncodeStruct.sol";
import "@xcall/utils/Types.sol";

import "@iconfoundation/btp2-solidity-library/interfaces/IBSH.sol";
import "@iconfoundation/btp2-solidity-library/interfaces/ICallService.sol";
import "@iconfoundation/btp2-solidity-library/interfaces/ICallServiceReceiver.sol";
import "@iconfoundation/btp2-solidity-library/interfaces/IDefaultCallServiceReceiver.sol";
import "@iconfoundation/btp2-solidity-library/utils/NetworkAddress.sol";
import "@iconfoundation/btp2-solidity-library/utils/Integers.sol";
import "@iconfoundation/btp2-solidity-library/utils/ParseAddress.sol";
import "@iconfoundation/btp2-solidity-library/utils/Strings.sol";
import "openzeppelin-contracts-upgradeable/contracts/proxy/utils/Initializable.sol";


contract CallService is IBSH, ICallService, IFeeManage, Initializable {
    using Strings for string;
    using Integers for uint;
    using ParseAddress for address;
    using ParseAddress for string;
    using NetworkAddress for string;
    using RLPEncodeStruct for Types.CSMessage;
    using RLPEncodeStruct for Types.CSMessageRequest;
    using RLPEncodeStruct for Types.CSMessageResponse;
    using RLPDecodeStruct for bytes;

    uint256 private constant MAX_DATA_SIZE = 2048;
    uint256 private constant MAX_ROLLBACK_SIZE = 1024;
    string private nid;
    string private networkAddress;
    uint256 private lastSn;
    uint256 private lastReqId;
    uint256 private protocolFee;

    mapping(uint256 => Types.CallRequest) private requests;
    mapping(uint256 => Types.ProxyRequest) private proxyReqs;

    mapping(uint256 => bool) private successfulResponses;

    mapping(bytes32 => mapping(string => bool)) private pendingReqs;
    mapping(uint256 => mapping(string => bool)) private pendingResponses;

    mapping(string => address) private defaultConnections;

    address private owner;
    address private adminAddress;
    address payable private feeHandler;

    modifier onlyOwner() {
        require(msg.sender == owner, "OnlyOwner");
        _;
    }

    modifier onlyAdmin() {
        require(msg.sender == _admin(), "OnlyAdmin");
        _;
    }

    function initialize(
        string memory _nid
    ) public initializer {
        owner = msg.sender;
        adminAddress = msg.sender;
        nid = _nid;
        networkAddress = nid.networkAddress(address(this).toString());
    }

    /* Implementation-specific external */
    function getNetworkAddress(
    ) external view override returns (
        string memory
    ) {
        return networkAddress;
    }

    function getNetworkId(
    ) external view override returns (
        string memory
    ) {
        return nid;
    }

    function checkService(
        string calldata _svc
    ) internal pure {
        require(Types.NAME.compareTo(_svc), "InvalidServiceName");
    }

    function getNextSn(
    ) internal returns (uint256) {
        lastSn = lastSn + 1;
        return lastSn;
    }

    function getNextReqId(
    ) internal returns (uint256) {
        lastReqId = lastReqId + 1;
        return lastReqId;
    }

    function cleanupCallRequest(
        uint256 sn
    ) internal {
        delete requests[sn];
    }

    function sendCallMessage(
        string memory _to,
        bytes memory _data,
        bytes memory _rollback,
        string[] memory sources,
        string[] memory destinations
    ) external payable override returns (
        uint256
    ) {
        return _sendCallMessage(_to, _data, _rollback, sources, destinations);
    }


    function sendCallMessage(
        string memory _to,
        bytes memory _data,
        bytes memory _rollback
    ) external payable override returns (
        uint256
    ) {
        string[] memory src;
        string[] memory dst;
        return _sendCallMessage(_to, _data, _rollback, src, dst);
    }

     function _sendCallMessage(
        string memory _to,
        bytes memory _data,
        bytes memory _rollback,
        string[] memory sources,
        string[] memory destinations
    ) internal returns (
        uint256
    ) {
        // check if caller is a contract or rollback data is null in case of EOA
        require(msg.sender.code.length > 0 || _rollback.length == 0, "RollbackNotPossible");

        // check size of payloads to avoid abusing
        require(_rollback.length <= MAX_ROLLBACK_SIZE, "MaxRollbackSizeExceeded");

        bool needResponse = _rollback.length > 0;
        (string memory netTo, string memory dstAccount) = _to.parseNetworkAddress();
        string memory from = nid.networkAddress(msg.sender.toString());
        uint256 sn = getNextSn();
        int256 msgSn = 0;
        if (needResponse) {
            requests[sn] = Types.CallRequest(msg.sender, netTo, sources, _rollback, false);
            msgSn = int256(sn);
        }
        Types.CSMessageRequest memory reqMsg = Types.CSMessageRequest(
            from, dstAccount, sn, needResponse, _data, destinations);
        bytes memory _msg = reqMsg.encodeCSMessageRequest();
        require(_msg.length <= MAX_DATA_SIZE, "MaxDataSizeExceeded");

        if (sources.length == 0) {
            address conn = defaultConnections[netTo];
            require(conn != address(0), "NoDefaultConnection");
            uint256 requiredFee = IConnection(conn).getFee(netTo, needResponse);
            sendBTPMessage(conn, requiredFee, netTo, Types.CS_REQUEST, msgSn, _msg);
        } else {
            for (uint i = 0; i < sources.length; i++) {
                address conn = sources[i].parseAddress("IllegalArgument");
                uint256 requiredFee = IConnection(conn).getFee(netTo, needResponse);
                sendBTPMessage(conn, requiredFee, netTo, Types.CS_REQUEST, msgSn, _msg);
            }
        }

        // handle protocol fee
        if (feeHandler != address(0) && address(this).balance > 0) {
            // we trust fee handler, it should just accept the protocol fee and return
            // assume that no reentrant cases occur here
            feeHandler.transfer(address(this).balance);
        }

        emit CallMessageSent(msg.sender, _to, sn);

        return sn;
    }

    function executeCall(
        uint256 _reqId,
        bytes memory _data
    ) external override {
        Types.ProxyRequest memory msgReq = proxyReqs[_reqId];
        require(bytes(msgReq.from).length > 0, "InvalidRequestId");
        require(msgReq.hash == keccak256(_data), "DataHashMismatch");
        // cleanup
        delete proxyReqs[_reqId];

        string memory netFrom = msgReq.from.nid();
        Types.CSMessageResponse memory msgRes;
        string memory errorMessage = "";
        try this.tryHandleCallMessage(
            address(0),
            msgReq.to,
            msgReq.from,
            _data,
            msgReq.protocols
        ) {
            msgRes = Types.CSMessageResponse(msgReq.sn, Types.CS_RESP_SUCCESS);
        } catch Error(string memory reason) {
            msgRes = Types.CSMessageResponse(msgReq.sn, Types.CS_RESP_FAILURE);
            errorMessage = reason;
        } catch (bytes memory) {
            msgRes = Types.CSMessageResponse(msgReq.sn, Types.CS_RESP_FAILURE);
            errorMessage = "unknownError";
        }
        emit CallExecuted(_reqId, msgRes.code, errorMessage);

        // send response only when there was a rollback
        if (msgReq.rollback) {
            if (msgReq.protocols.length == 0) {
                address conn = defaultConnections[netFrom];
                sendBTPMessage(conn, 0, netFrom, Types.CS_RESPONSE, int256(msgReq.sn) * - 1, msgRes.encodeCSMessageResponse());

            } else {
                for (uint i = 0; i < msgReq.protocols.length; i++) {
                    address conn = msgReq.protocols[i].parseAddress("IllegalArgument");
                    sendBTPMessage(conn, 0, netFrom, Types.CS_RESPONSE, int256(msgReq.sn) * - 1, msgRes.encodeCSMessageResponse());
                }
            }

        }
    }

    //  @dev To catch error
    function tryHandleCallMessage(
        address toAddr,
        string memory to,
        string memory from,
        bytes memory data,
        string[] memory protocols
    ) external {
        require(msg.sender == address(this), "OnlyInternal");
        if (toAddr == address(0)) {
            toAddr = to.parseAddress("IllegalArgument");
        }
        if (protocols.length == 0) {
            IDefaultCallServiceReceiver(toAddr).handleCallMessage(from, data);
        } else {
            ICallServiceReceiver(toAddr).handleCallMessage(from, data, protocols);
        }
    }

    function executeRollback(
        uint256 _sn
    ) external override {
        Types.CallRequest memory req = requests[_sn];
        require(req.from != address(0), "InvalidSerialNum");
        require(req.enabled, "RollbackNotEnabled");
        cleanupCallRequest(_sn);

        this.tryHandleCallMessage(
            req.from,
            "",
            networkAddress,
            req.rollback,
            req.sources
        );

        emit RollbackExecuted(_sn);
    }

    /* ========== Interfaces with BMC ========== */
    function handleBTPMessage(
        string calldata _from,
        string calldata _svc,
        uint256 _sn,
        bytes calldata _msg
    ) external override {
        checkService(_svc);
        handleMessage(_from, _msg);
    }

    function handleBTPError(
        string calldata _src,
        string calldata _svc,
        uint256 _sn,
        uint256 _code,
        string calldata _msg
    ) external override {
        checkService(_svc);
        handleError(_sn);
    }
    /* ========================================= */

    function handleMessage(
        string calldata _from,
        bytes calldata _msg
    ) public override {
        require(!_from.compareTo(nid), "Invalid Network ID");
        Types.CSMessage memory csMsg = _msg.decodeCSMessage();
        if (csMsg.msgType == Types.CS_REQUEST) {
            handleRequest(_from, csMsg.payload);
        } else if (csMsg.msgType == Types.CS_RESPONSE) {
            handleResponse(csMsg.payload.decodeCSMessageResponse());
        } else {
            string memory errMsg = string("UnknownMsgType(")
                .concat(uint(csMsg.msgType).toString())
                .concat(string(")"));
            revert(errMsg);
        }
    }

    function handleError(
        uint256 _sn
    ) public override {
        handleResponse(Types.CSMessageResponse(
            _sn,
            Types.CS_RESP_FAILURE
        ));
    }

    function sendBTPMessage(
        address connection,
        uint256 value,
        string memory netTo,
        int msgType,
        int256 sn,
        bytes memory msgPayload
    ) internal {
        IConnection(connection).sendMessage{value: value}(
            netTo,
            Types.NAME,
            sn,
            Types.CSMessage(
                msgType,
                msgPayload
            ).encodeCSMessage()
        );
    }

    function handleRequest(
        string memory netFrom,
        bytes memory msgPayload
    ) internal {
        Types.CSMessageRequest memory req = msgPayload.decodeCSMessageRequest();
        string memory fromNID = req.from.nid();
        require(netFrom.compareTo(fromNID),"Invalid NID");

        bytes32 dataHash = keccak256(req.data);
        if (req.protocols.length > 1) {
            pendingReqs[dataHash][msg.sender.toString()] = true;
            for (uint i = 0; i < req.protocols.length; i++) {
                if (!pendingReqs[dataHash][req.protocols[i]]) {
                    return;
                }
            }

            for (uint i = 0; i < req.protocols.length; i++) {
                delete pendingReqs[dataHash][req.protocols[i]];
            }
        } else if (req.protocols.length == 1) {
            require(msg.sender == req.protocols[0].parseAddress("IllegalArgument"), "NotAuthorized");
        } else {
            require(msg.sender == defaultConnections[fromNID], "NotAuthorized");
        }

        uint256 reqId = getNextReqId();
        proxyReqs[reqId] = Types.ProxyRequest(
            req.from,
            req.to,
            req.sn,
            req.rollback,
            dataHash,
            req.protocols
        );

        emit CallMessage(req.from, req.to, req.sn, reqId, req.data);
    }

    function handleResponse(
        Types.CSMessageResponse memory res
    ) internal {
        Types.CallRequest memory req = requests[res.sn];
        if (req.from == address(0)) {
            return;
        }

        if (req.sources.length > 1) {
            pendingResponses[res.sn][msg.sender.toString()] = true;
            for (uint i = 0; i < req.sources.length; i++) {
                if (!pendingResponses[res.sn][req.sources[i]]) {
                    return;
                }
            }

            for (uint i = 0; i < req.sources.length; i++) {
                delete pendingResponses[res.sn][req.sources[i]];
            }
        } else if (req.sources.length == 1) {
            require(msg.sender == req.sources[0].parseAddress("IllegalArgument"), "NotAuthorized");
        } else {
            require(msg.sender == defaultConnections[req.to], "NotAuthorized");
        }

        emit ResponseMessage(res.sn, res.code);
        if (res.code == Types.CS_RESP_SUCCESS) {
            cleanupCallRequest(res.sn);
            successfulResponses[res.sn] = true;
        } else {
            //emit rollback event
            require(req.rollback.length > 0, "NoRollbackData");
            req.enabled = true;
            requests[res.sn] = req;
            emit RollbackMessage(res.sn);
        }
    }

    function _admin(
    ) internal view returns (
        address
    ) {
        if (adminAddress == address(0)) {
            return owner;
        }
        return adminAddress;
    }

    /**
       @notice Gets the address of admin
       @return (Address) the address of admin
    */
    function admin(
    ) external view returns (
        address
    ) {
        return _admin();
    }

    /**
       @notice Sets the address of admin
       @dev Only the owner wallet can invoke this.
       @param _address (Address) The address of admin
    */
    function setAdmin(
        address _address
    ) external onlyAdmin {
        adminAddress = _address;
    }

    function setProtocolFeeHandler(
        address _addr
    ) external override onlyAdmin {
        feeHandler = payable(_addr);
    }

    function getProtocolFeeHandler(
    ) external view override returns (
        address
    ) {
        return feeHandler;
    }

    function setDefaultConnection(string memory _nid, address connection) external onlyAdmin {
        defaultConnections[_nid] = connection;
    }

    function getDefaultConnection(string memory _nid) external view returns (address) {
        return defaultConnections[_nid];
    }

    function setProtocolFee(
        uint256 _value
    ) external override onlyAdmin {
        require(_value >= 0, "ValueShouldBePositive");
        protocolFee = _value;
    }

    function getProtocolFee(
    ) external view override returns (
        uint256
    ) {
        return protocolFee;
    }

    function _getFee(
        address connection,
        string memory _net,
        bool _rollback
    ) internal view returns (
        uint256
    ) {
        return IConnection(connection).getFee(_net, _rollback);
    }

    function getFee(
        string memory _net,
        bool _rollback
    ) external view override returns (
        uint256
    ) {
        return protocolFee + _getFee(defaultConnections[_net], _net, _rollback);
    }

    function getFee(
        string memory _net,
        bool _rollback,
        string[] memory _sources
    ) external view override returns (
        uint256
    ) {
        uint256 fee = protocolFee;
        for (uint i = 0; i < _sources.length; i++) {
            address conn = _sources[i].parseAddress("IllegalArgument");
            fee = fee + _getFee(conn, _net, _rollback);
        }

        return fee;
    }

    function verifySuccess(uint256 _sn) external view returns (bool) {
        return successfulResponses[_sn];
    }
}
