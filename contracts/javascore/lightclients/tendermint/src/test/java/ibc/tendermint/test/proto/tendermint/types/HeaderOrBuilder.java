// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tendermint/types/types.proto

package com.tendermint.types;

public interface HeaderOrBuilder extends
    // @@protoc_insertion_point(interface_extends:tendermint.types.Header)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <pre>
   * basic block info
   * </pre>
   *
   * <code>.tendermint.version.Consensus version = 1 [json_name = "version", (.gogoproto.nullable) = false];</code>
   * @return Whether the version field is set.
   */
  boolean hasVersion();
  /**
   * <pre>
   * basic block info
   * </pre>
   *
   * <code>.tendermint.version.Consensus version = 1 [json_name = "version", (.gogoproto.nullable) = false];</code>
   * @return The version.
   */
  com.tendermint.version.Consensus getVersion();

  /**
   * <code>string chain_id = 2 [json_name = "chainId", (.gogoproto.customname) = "ChainID"];</code>
   * @return The chainId.
   */
  java.lang.String getChainId();
  /**
   * <code>string chain_id = 2 [json_name = "chainId", (.gogoproto.customname) = "ChainID"];</code>
   * @return The bytes for chainId.
   */
  com.google.protobuf.ByteString
      getChainIdBytes();

  /**
   * <code>int64 height = 3 [json_name = "height"];</code>
   * @return The height.
   */
  long getHeight();

  /**
   * <code>.google.protobuf.Timestamp time = 4 [json_name = "time", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   * @return Whether the time field is set.
   */
  boolean hasTime();
  /**
   * <code>.google.protobuf.Timestamp time = 4 [json_name = "time", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   * @return The time.
   */
  com.google.protobuf.Timestamp getTime();

  /**
   * <pre>
   * prev block info
   * </pre>
   *
   * <code>.tendermint.types.BlockID last_block_id = 5 [json_name = "lastBlockId", (.gogoproto.nullable) = false];</code>
   * @return Whether the lastBlockId field is set.
   */
  boolean hasLastBlockId();
  /**
   * <pre>
   * prev block info
   * </pre>
   *
   * <code>.tendermint.types.BlockID last_block_id = 5 [json_name = "lastBlockId", (.gogoproto.nullable) = false];</code>
   * @return The lastBlockId.
   */
  com.tendermint.types.BlockID getLastBlockId();

  /**
   * <pre>
   * hashes of block data
   * </pre>
   *
   * <code>bytes last_commit_hash = 6 [json_name = "lastCommitHash"];</code>
   * @return The lastCommitHash.
   */
  com.google.protobuf.ByteString getLastCommitHash();

  /**
   * <pre>
   * transactions
   * </pre>
   *
   * <code>bytes data_hash = 7 [json_name = "dataHash"];</code>
   * @return The dataHash.
   */
  com.google.protobuf.ByteString getDataHash();

  /**
   * <pre>
   * hashes from the app output from the prev block
   * </pre>
   *
   * <code>bytes validators_hash = 8 [json_name = "validatorsHash"];</code>
   * @return The validatorsHash.
   */
  com.google.protobuf.ByteString getValidatorsHash();

  /**
   * <pre>
   * validators for the next block
   * </pre>
   *
   * <code>bytes next_validators_hash = 9 [json_name = "nextValidatorsHash"];</code>
   * @return The nextValidatorsHash.
   */
  com.google.protobuf.ByteString getNextValidatorsHash();

  /**
   * <pre>
   * consensus params for current block
   * </pre>
   *
   * <code>bytes consensus_hash = 10 [json_name = "consensusHash"];</code>
   * @return The consensusHash.
   */
  com.google.protobuf.ByteString getConsensusHash();

  /**
   * <pre>
   * state after txs from the previous block
   * </pre>
   *
   * <code>bytes app_hash = 11 [json_name = "appHash"];</code>
   * @return The appHash.
   */
  com.google.protobuf.ByteString getAppHash();

  /**
   * <pre>
   * root hash of all results from the txs from the previous block
   * </pre>
   *
   * <code>bytes last_results_hash = 12 [json_name = "lastResultsHash"];</code>
   * @return The lastResultsHash.
   */
  com.google.protobuf.ByteString getLastResultsHash();

  /**
   * <pre>
   * consensus info
   * </pre>
   *
   * <code>bytes evidence_hash = 13 [json_name = "evidenceHash"];</code>
   * @return The evidenceHash.
   */
  com.google.protobuf.ByteString getEvidenceHash();

  /**
   * <pre>
   * original proposer of the block
   * </pre>
   *
   * <code>bytes proposer_address = 14 [json_name = "proposerAddress"];</code>
   * @return The proposerAddress.
   */
  com.google.protobuf.ByteString getProposerAddress();
}
