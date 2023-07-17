// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedHeader {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<BtpHeader>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub current_validators: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag="4")]
    pub trusted_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtpHeader {
    #[prost(uint64, tag="1")]
    pub main_height: u64,
    #[prost(uint32, tag="2")]
    pub round: u32,
    #[prost(bytes="vec", tag="3")]
    pub next_proof_context_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub network_section_to_root: ::prost::alloc::vec::Vec<MerkleNode>,
    #[prost(uint64, tag="5")]
    pub network_id: u64,
    #[prost(uint64, tag="6")]
    pub update_number: u64,
    #[prost(bytes="vec", tag="7")]
    pub prev_network_section_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="8")]
    pub message_count: u64,
    #[prost(bytes="vec", tag="9")]
    pub message_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="10")]
    pub next_validators: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleNode {
    #[prost(int32, tag="1")]
    pub dir: i32,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleProofs {
    #[prost(message, repeated, tag="1")]
    pub proofs: ::prost::alloc::vec::Vec<MerkleNode>,
}
/// BlockIdFlag indicates which BlcokID the signature is for
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockIdFlag {
    BlockIdFlagUnknown = 0,
    BlockIdFlagAbsent = 1,
    BlockIdFlagCommit = 2,
    BlockIdFlagNil = 3,
}
impl BlockIdFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockIdFlag::BlockIdFlagUnknown => "BLOCK_ID_FLAG_UNKNOWN",
            BlockIdFlag::BlockIdFlagAbsent => "BLOCK_ID_FLAG_ABSENT",
            BlockIdFlag::BlockIdFlagCommit => "BLOCK_ID_FLAG_COMMIT",
            BlockIdFlag::BlockIdFlagNil => "BLOCK_ID_FLAG_NIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_ID_FLAG_UNKNOWN" => Some(Self::BlockIdFlagUnknown),
            "BLOCK_ID_FLAG_ABSENT" => Some(Self::BlockIdFlagAbsent),
            "BLOCK_ID_FLAG_COMMIT" => Some(Self::BlockIdFlagCommit),
            "BLOCK_ID_FLAG_NIL" => Some(Self::BlockIdFlagNil),
            _ => None,
        }
    }
}
/// SignedMsgType is a type of signed message in the consensus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignedMsgType {
    SignedMsgTypeUnknown = 0,
    /// Votes
    SignedMsgTypePrevote = 1,
    SignedMsgTypePrecommit = 2,
    /// Proposals
    SignedMsgTypeProposal = 32,
}
impl SignedMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignedMsgType::SignedMsgTypeUnknown => "SIGNED_MSG_TYPE_UNKNOWN",
            SignedMsgType::SignedMsgTypePrevote => "SIGNED_MSG_TYPE_PREVOTE",
            SignedMsgType::SignedMsgTypePrecommit => "SIGNED_MSG_TYPE_PRECOMMIT",
            SignedMsgType::SignedMsgTypeProposal => "SIGNED_MSG_TYPE_PROPOSAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNED_MSG_TYPE_UNKNOWN" => Some(Self::SignedMsgTypeUnknown),
            "SIGNED_MSG_TYPE_PREVOTE" => Some(Self::SignedMsgTypePrevote),
            "SIGNED_MSG_TYPE_PRECOMMIT" => Some(Self::SignedMsgTypePrecommit),
            "SIGNED_MSG_TYPE_PROPOSAL" => Some(Self::SignedMsgTypeProposal),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `icon.types.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf7, 0x16, 0x0a, 0x19, 0x69, 0x63, 0x6f, 0x6e, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f,
    0x76, 0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d,
    0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x22, 0xb5, 0x01,
    0x0a, 0x0c, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x30,
    0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18,
    0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x42,
    0x54, 0x50, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0c, 0x52, 0x0a, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73,
    0x12, 0x2c, 0x0a, 0x11, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x11, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x12, 0x25,
    0x0a, 0x0e, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x48,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x22, 0xba, 0x03, 0x0a, 0x09, 0x42, 0x54, 0x50, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x61, 0x69, 0x6e, 0x5f, 0x68, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x6d, 0x61, 0x69, 0x6e, 0x48, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x05, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x35, 0x0a, 0x17, 0x6e, 0x65,
    0x78, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x6f, 0x66, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74,
    0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x14, 0x6e, 0x65, 0x78,
    0x74, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x48, 0x61, 0x73,
    0x68, 0x12, 0x50, 0x0a, 0x17, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x73, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x6f, 0x5f, 0x72, 0x6f, 0x6f, 0x74, 0x18, 0x04, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x19, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e,
    0x76, 0x31, 0x2e, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x14, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x6f, 0x52,
    0x6f, 0x6f, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69,
    0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x65, 0x76, 0x5f,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x68, 0x61, 0x73, 0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x16, 0x70, 0x72, 0x65, 0x76,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x53, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x61,
    0x73, 0x68, 0x12, 0x23, 0x0a, 0x0d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x5f, 0x72, 0x6f, 0x6f, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x6f, 0x6f, 0x74, 0x12, 0x26, 0x0a, 0x0e, 0x6e, 0x65,
    0x78, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x0a, 0x20, 0x03,
    0x28, 0x0c, 0x52, 0x0e, 0x6e, 0x65, 0x78, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f,
    0x72, 0x73, 0x22, 0x34, 0x0a, 0x0a, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x4e, 0x6f, 0x64, 0x65,
    0x12, 0x10, 0x0a, 0x03, 0x44, 0x69, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x03, 0x44,
    0x69, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x41, 0x0a, 0x0c, 0x4d, 0x65, 0x72, 0x6b,
    0x6c, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x73, 0x12, 0x31, 0x0a, 0x06, 0x70, 0x72, 0x6f, 0x6f,
    0x66, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x4e,
    0x6f, 0x64, 0x65, 0x52, 0x06, 0x70, 0x72, 0x6f, 0x6f, 0x66, 0x73, 0x2a, 0x73, 0x0a, 0x0b, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x44, 0x46, 0x6c, 0x61, 0x67, 0x12, 0x19, 0x0a, 0x15, 0x42, 0x4c,
    0x4f, 0x43, 0x4b, 0x5f, 0x49, 0x44, 0x5f, 0x46, 0x4c, 0x41, 0x47, 0x5f, 0x55, 0x4e, 0x4b, 0x4e,
    0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x14, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x5f, 0x49,
    0x44, 0x5f, 0x46, 0x4c, 0x41, 0x47, 0x5f, 0x41, 0x42, 0x53, 0x45, 0x4e, 0x54, 0x10, 0x01, 0x12,
    0x18, 0x0a, 0x14, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x5f, 0x49, 0x44, 0x5f, 0x46, 0x4c, 0x41, 0x47,
    0x5f, 0x43, 0x4f, 0x4d, 0x4d, 0x49, 0x54, 0x10, 0x02, 0x12, 0x15, 0x0a, 0x11, 0x42, 0x4c, 0x4f,
    0x43, 0x4b, 0x5f, 0x49, 0x44, 0x5f, 0x46, 0x4c, 0x41, 0x47, 0x5f, 0x4e, 0x49, 0x4c, 0x10, 0x03,
    0x2a, 0x86, 0x01, 0x0a, 0x0d, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x4d, 0x73, 0x67, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x1b, 0x0a, 0x17, 0x53, 0x49, 0x47, 0x4e, 0x45, 0x44, 0x5f, 0x4d, 0x53, 0x47,
    0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12,
    0x1b, 0x0a, 0x17, 0x53, 0x49, 0x47, 0x4e, 0x45, 0x44, 0x5f, 0x4d, 0x53, 0x47, 0x5f, 0x54, 0x59,
    0x50, 0x45, 0x5f, 0x50, 0x52, 0x45, 0x56, 0x4f, 0x54, 0x45, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19,
    0x53, 0x49, 0x47, 0x4e, 0x45, 0x44, 0x5f, 0x4d, 0x53, 0x47, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f,
    0x50, 0x52, 0x45, 0x43, 0x4f, 0x4d, 0x4d, 0x49, 0x54, 0x10, 0x02, 0x12, 0x1c, 0x0a, 0x18, 0x53,
    0x49, 0x47, 0x4e, 0x45, 0x44, 0x5f, 0x4d, 0x53, 0x47, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x50,
    0x52, 0x4f, 0x50, 0x4f, 0x53, 0x41, 0x4c, 0x10, 0x20, 0x42, 0x94, 0x01, 0x0a, 0x11, 0x63, 0x6f,
    0x6d, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x42,
    0x0a, 0x54, 0x79, 0x70, 0x65, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x1d, 0x6c,
    0x69, 0x62, 0x72, 0x61, 0x72, 0x69, 0x65, 0x73, 0x2f, 0x67, 0x6f, 0x2f, 0x63, 0x6f, 0x6d, 0x6d,
    0x6f, 0x6e, 0x2f, 0x69, 0x63, 0x6f, 0x6e, 0x3b, 0x69, 0x63, 0x6f, 0x6e, 0xa2, 0x02, 0x03, 0x49,
    0x54, 0x58, 0xaa, 0x02, 0x0d, 0x49, 0x63, 0x6f, 0x6e, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x73, 0x2e,
    0x56, 0x31, 0xca, 0x02, 0x0d, 0x49, 0x63, 0x6f, 0x6e, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x73, 0x5c,
    0x56, 0x31, 0xe2, 0x02, 0x19, 0x49, 0x63, 0x6f, 0x6e, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x73, 0x5c,
    0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02,
    0x0f, 0x49, 0x63, 0x6f, 0x6e, 0x3a, 0x3a, 0x54, 0x79, 0x70, 0x65, 0x73, 0x3a, 0x3a, 0x56, 0x31,
    0x4a, 0xbf, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x16, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x34, 0x0a, 0x41, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x03, 0x07, 0x00, 0x34, 0x1a, 0x14, 0x20, 0x67, 0x6f, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67,
    0x65, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x0a, 0x32, 0x20, 0x20, 0x69, 0x6d, 0x70,
    0x6f, 0x72, 0x74, 0x20, 0x22, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67,
    0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3b, 0x0a, 0x0a, 0x46, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x11, 0x03, 0x1a, 0x3a, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x49, 0x64, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x42, 0x6c, 0x63, 0x6f, 0x6b, 0x49, 0x44, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20,
    0x66, 0x6f, 0x72, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x05, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x0e, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0e, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0e,
    0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x10, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x10, 0x1c, 0x1d, 0x0a, 0x49, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x15, 0x00, 0x1e, 0x01,
    0x1a, 0x3d, 0x20, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x4d, 0x73, 0x67, 0x54, 0x79, 0x70, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x69,
    0x67, 0x6e, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x15, 0x05, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x17, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x17, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x17, 0x1e, 0x1f, 0x0a, 0x14, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x19, 0x04,
    0x23, 0x1a, 0x07, 0x20, 0x56, 0x6f, 0x74, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x19, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x1a, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x04,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x20, 0x21, 0x0a,
    0x18, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x04, 0x23, 0x1a, 0x0b, 0x20, 0x50,
    0x72, 0x6f, 0x70, 0x6f, 0x73, 0x61, 0x6c, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1d, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x1d, 0x1f, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x20, 0x00, 0x25,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x20, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x21, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x21, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x21, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x04,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x22, 0x0e, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x23, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23,
    0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x16, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x23, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x24, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x24, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x24, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x24, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x27, 0x00, 0x33,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x27, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x28, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x28, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x29, 0x04,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x10, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x2a, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x2a, 0x10, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a,
    0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x04, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2b, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2b, 0x1b, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x2b, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12,
    0x03, 0x2c, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2c,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2c, 0x14, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2c, 0x26, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x2d, 0x14, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x2d, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x2e,
    0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x2e, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2e, 0x14, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2e, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x2f, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x2f, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x2f, 0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x2f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x30, 0x04, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x30, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x30, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x30, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x09, 0x12, 0x03, 0x31, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x31, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x31, 0x16,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x31, 0x27, 0x29, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x35, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x35, 0x09, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x36, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x36,
    0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x0a, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x37, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x37, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x37, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3a, 0x00, 0x3c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3b, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x3b, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x3b, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b,
    0x20, 0x21, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("icon.types.v1.serde.rs");
// @@protoc_insertion_point(module)