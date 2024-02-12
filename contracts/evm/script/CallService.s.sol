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

    string internal nid;
    uint256 internal protocolFee;

    constructor() {
        deployerPrivateKey = vm.envUint("PRIVATE_KEY");
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
            console2.log("Wormhole Adapter address:", proxy, "\n");
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
            console2.log("LayerZero Adapter address:", proxy, "\n");
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
            console2.log("Centralized Connection address:", proxy, "\n");
        } else if (contractA.compareTo("mock")) {
            address xcall = vm.envAddress(chain.concat("_XCALL"));
            address proxy = Upgrades.deployTransparentProxy(
                "MultiProtocolSampleDapp.sol",
                msg.sender,
                abi.encodeCall(MultiProtocolSampleDapp.initialize, xcall)
            );
            console2.log("Mock Dapp address:", proxy, "\n");
        }
    }

    function configureWormholeConnection(
        string memory chain1,
        string memory chain2
    ) public broadcast(deployerPrivateKey) {
        address chain1_adapter = vm.envAddress(
            capitalizeString(chain1).concat("_WORMHOLE_ADAPTER")
        );
        address chain2_adapter = vm.envAddress(
            capitalizeString(chain2).concat("_WORMHOLE_ADAPTER")
        );

        string memory nid_chain1 = vm.envString(
            capitalizeString(chain1).concat("_NID")
        );
        string memory nid_chain2 = vm.envString(
            capitalizeString(chain2).concat("_NID")
        );

        uint256 chain1_id = vm.envUint(
            capitalizeString(chain1).concat("_CHAIN_ID")
        );
        uint256 chain2_id = vm.envUint(
            capitalizeString(chain2).concat("_CHAIN_ID")
        );
        console2.log(chain1_id, chain2_id, chain1_adapter, chain2_adapter);
        WormholeAdapter adapter1 = WormholeAdapter(chain1_adapter);
        WormholeAdapter adapter2 = WormholeAdapter(chain2_adapter);

        adapter1.configureConnection(
            nid_chain2,
            uint16(chain2_id),
            toWormholeFormat(chain2_adapter),
            5_000_000,
            uint256(1e14)
        );
    }

    function configureLayerzeroConnection(
        string memory chain1,
        string memory chain2
    ) public broadcast(deployerPrivateKey) {
        address chain1_adapter = vm.envAddress(
            capitalizeString(chain1).concat("_LAYERZERO_ADAPTER")
        );
        address chain2_adapter = vm.envAddress(
            capitalizeString(chain2).concat("_LAYERZERO_ADAPTER")
        );

        string memory nid_chain1 = vm.envString(
            capitalizeString(chain1).concat("_NID")
        );
        string memory nid_chain2 = vm.envString(
            capitalizeString(chain2).concat("_NID")
        );

        uint256 chain1_id = vm.envUint(
            capitalizeString(chain1).concat("_LAYERZERO_CHAIN_ID")
        );
        uint256 chain2_id = vm.envUint(
            capitalizeString(chain2).concat("_LAYERZERO_CHAIN_ID")
        );

        LayerZeroAdapter adapter1 = LayerZeroAdapter(payable(chain1_adapter));
        LayerZeroAdapter adapter2 = LayerZeroAdapter(payable(chain2_adapter));

        adapter1.configureConnection(
            nid_chain2,
            uint16(chain2_id),
            abi.encodePacked(chain2_adapter),
            uint256(900_000),
            uint256(1e10)
        );
    }

    function addConnection(
        string memory chain1,
        string memory chain2
    ) public broadcast(deployerPrivateKey) {
        address chain1_dapp = vm.envAddress(
            capitalizeString(chain1).concat("_MOCK_DAPP")
        );
        address chain2_dapp = vm.envAddress(
            capitalizeString(chain2).concat("_MOCK_DAPP")
        );

        address chain1_adapter = vm.envAddress(
            capitalizeString(chain1).concat("_WORMHOLE_ADAPTER")
        );
        address chain2_adapter = vm.envAddress(
            capitalizeString(chain2).concat("_WORMHOLE_ADAPTER")
        );

        string memory nid_chain1 = vm.envString(
            capitalizeString(chain1).concat("_NID")
        );
        string memory nid_chain2 = vm.envString(
            capitalizeString(chain2).concat("_NID")
        );

        MultiProtocolSampleDapp dapp1 = MultiProtocolSampleDapp(chain1_dapp);
        MultiProtocolSampleDapp dapp2 = MultiProtocolSampleDapp(chain2_dapp);

        dapp1.addConnection(
            nid_chain2,
            ParseAddress.toString(chain1_adapter),
            ParseAddress.toString(chain2_adapter)
        );
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
