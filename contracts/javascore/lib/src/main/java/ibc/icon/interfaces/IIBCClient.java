package ibc.icon.interfaces;

import score.Address;

import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;

public interface IIBCClient {
    /**
     * @dev registerClient registers a new client type into the client registry
     */
    void registerClient(String clientType, Address lightClient);

    /**
     * @dev createClient creates a new client state and populates it with a given consensus state
     */
    void createClient(MsgCreateClient msg);

    /**
     * @dev updateClient updates the consensus state and the state root from a provided header
     */
    void updateClient(MsgUpdateClient msg);
}
