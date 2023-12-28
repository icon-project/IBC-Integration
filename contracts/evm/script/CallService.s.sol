// SPDX-License-Identifier: MIT
pragma solidity >=0.8.18;
import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";

import "@xcall/contracts/xcall/CallService.sol";
import "@xcall/contracts/mocks/multi-protocol-dapp/MultiProtocolSampleDapp.sol";
import "@xcall/contracts/adapters/WormholeAdapter.sol";
import "@xcall/contracts/adapters/LayerZeroAdapter.sol";
import "@xcall/contracts/adapters/CentralizedConnection.sol";

contract DeployCallService is Script {
    CallService private proxyXcall;
    CallService private wrappedProxy;

    using Strings for string;

    uint256 internal deployerPrivateKey;
    address internal ownerAddress;

    string internal nid;
    uint256 internal protocolFee;

    constructor() {
        deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        ownerAddress = vm.envAddress("OWNER_ADDRESS");
    }

    modifier broadcast(uint256 privateKey) {
        vm.startBroadcast(privateKey);

        _;

        vm.stopBroadcast();
    }

    function capitalizeString(
        string memory input
    ) public pure returns (string memory) {
        bytes memory inputBytes = bytes(input);
        for (uint i = 0; i < inputBytes.length; i++) {
            if (uint8(inputBytes[i]) >= 97 && uint8(inputBytes[i]) <= 122) {
                inputBytes[i] = bytes1(uint8(inputBytes[i]) - 32);
            }
        }
        return string(inputBytes);
    }

    function deployContract(
        string memory env,
        string memory chain,
        string memory contractA
    ) external broadcast(deployerPrivateKey) {
        env = capitalizeString(env);
        chain = capitalizeString(chain);
        nid = vm.envString(chain.concat("_NID"));

        if (contractA.compareTo("callservice")) {
            address proxy = Upgrades.deployTransparentProxy(
                "CallService.sol",
                msg.sender,
                abi.encodeCall(CallService.initialize, nid)
            );
            console2.log("CallService address:", proxy, "\n");

            proxyXcall = CallService(proxy);
            proxyXcall.setProtocolFee(protocolFee);
            proxyXcall.setProtocolFeeHandler(ownerAddress);
        } else if (contractA.compareTo("wormhole")) {
            address xcall = vm.envAddress(chain.concat("_XCALL"));
            address wormholeRelayer = vm.envAddress(
                chain.concat("_WORMHOLE_RELAYER")
            );

            address proxy = Upgrades.deployTransparentProxy(
                "WormholeAdapter.sol",
                msg.sender,
                abi.encodeCall(
                    WormholeAdapter.initialize,
                    (wormholeRelayer, xcall)
                )
            );
        } else if (contractA.compareTo("layerzero")) {
            address xcall = vm.envAddress(chain.concat("_XCALL"));
            address layerzeroRelayer = vm.envAddress(
                chain.concat("_LAYERZERO_RELAYER")
            );

            address proxy = Upgrades.deployTransparentProxy(
                "LayerZeroAdapter.sol",
                msg.sender,
                abi.encodeCall(
                    LayerZeroAdapter.initialize,
                    (layerzeroRelayer, xcall)
                )
            );
        } else if (contractA.compareTo("centralized")) {
            address xcall = vm.envAddress(chain.concat("_XCALL"));
            address centralizedRelayer = vm.envAddress(
                chain.concat("_CENTRALIZED_RELAYER")
            );

            address proxy = Upgrades.deployTransparentProxy(
                "CentralizedConnection.sol",
                msg.sender,
                abi.encodeCall(
                    CentralizedConnection.initialize,
                    (centralizedRelayer, xcall)
                )
            );
        } else if(contractA.compareTo("mock")) {
            address xcall = vm.envAddress(chain.concat("_XCALL"));
            address proxy = Upgrades.deployTransparentProxy(
                "MultiProtocolSampleDapp.sol",
                msg.sender,
                abi.encodeCall(
                    MultiProtocolSampleDapp.initialize,
                    xcall
                )
            );
        }
    }

    function upgradeContract(
        string memory chain,
        string memory contractName,
        string memory contractA
    ) external broadcast(deployerPrivateKey) {
        if (contractA.compareTo("callservice")) {
            address proxy = vm.envAddress(
                capitalizeString(chain).concat("_XCALL")
            );
            Upgrades.upgradeProxy(proxy, contractName, "");
        } else if (contractA.compareTo("wormhole")) {
            address proxy = vm.envAddress(
                capitalizeString(chain).concat("_WORMHOLE_ADAPTER")
            );
            Upgrades.upgradeProxy(proxy, contractName, "");
        } else if (contractA.compareTo("layerzero")) {
            address proxy = vm.envAddress(
                capitalizeString(chain).concat("_LAYERZERO_ADAPTER")
            );
            Upgrades.upgradeProxy(proxy, contractName, "");
        } else if (contractA.compareTo("centralized")) {
            address proxy = vm.envAddress(
                capitalizeString(chain).concat("_CENTRALIZED_ADAPTER")
            );
            Upgrades.upgradeProxy(proxy, contractName, "");
        }
    }
}
