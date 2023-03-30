package ibc.icon.interfaces;

import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import score.Address;

public interface IIBCClient {
    /**
     * {@code @dev} registerClient registers a new client type into the client registry
     * @param clientType  Type of client
     * @param lightClient Light client contract address
     */
    void registerClient(String clientType, Address lightClient);

    /**
     * {@code @dev} createClient creates a new client state and populates it with a given
     * consensus state
     */
    String createClient(MsgCreateClient msg);

    /**
     * {@code @dev} updateClient updates the consensus state and the state root from a
     * provided header
     */
    void updateClient(MsgUpdateClient msg);
}
