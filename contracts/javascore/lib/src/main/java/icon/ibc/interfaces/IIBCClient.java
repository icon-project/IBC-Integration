package icon.ibc.interfaces;

import foundation.icon.score.client.ScoreClient;
import icon.ibc.structs.messages.MsgCreateClient;
import icon.ibc.structs.messages.MsgUpdateClient;
import score.Address;
import score.annotation.EventLog;
import score.annotation.Optional;

@ScoreClient
public interface IIBCClient {

    @EventLog(indexed = 1)
    public void CreateClient(String identifier, byte[] clientState);

    @EventLog(indexed = 1)
    public void UpdateClient(String identifier, byte[] consensusHeight, byte[] clientMessage);

    /**
     * {@code @dev} registerClient registers a new client type into the client registry
     * @param clientType  Type of client
     * @param lightClient Light client contract address
     * @param hashType Optional hashType (WASM or ICS08) default WASM
     */
    void registerClient(String clientType, Address client, @Optional int hashType);

    /**
     * {@code @dev} createClient creates a new client state and populates it with a given
     * consensus state
     */
    void createClient(MsgCreateClient msg);

    /**
     * {@code @dev} updateClient updates the consensus state and the state root from a
     * provided header
     */
    void updateClient(MsgUpdateClient msg);
}
