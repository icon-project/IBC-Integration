// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: icon/lightclient/v1/light.proto

package icon

import (
	fmt "fmt"
	proto "github.com/gogo/protobuf/proto"
	io "io"
	math "math"
	math_bits "math/bits"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.GoGoProtoPackageIsVersion3 // please upgrade the proto package

type ClientState struct {
	TrustingPeriod     uint64   `protobuf:"varint,1,opt,name=trusting_period,json=trustingPeriod,proto3" json:"trusting_period,omitempty"`
	FrozenHeight       uint64   `protobuf:"varint,2,opt,name=frozen_height,json=frozenHeight,proto3" json:"frozen_height,omitempty"`
	MaxClockDrift      uint64   `protobuf:"varint,3,opt,name=max_clock_drift,json=maxClockDrift,proto3" json:"max_clock_drift,omitempty"`
	LatestHeight       uint64   `protobuf:"varint,4,opt,name=latest_height,json=latestHeight,proto3" json:"latest_height,omitempty"`
	NetworkSectionHash []byte   `protobuf:"bytes,5,opt,name=network_section_hash,json=networkSectionHash,proto3" json:"network_section_hash,omitempty"`
	Validators         [][]byte `protobuf:"bytes,6,rep,name=validators,proto3" json:"validators,omitempty"`
}

func (m *ClientState) Reset()         { *m = ClientState{} }
func (m *ClientState) String() string { return proto.CompactTextString(m) }
func (*ClientState) ProtoMessage()    {}
func (*ClientState) Descriptor() ([]byte, []int) {
	return fileDescriptor_5ae86e09394aefe7, []int{0}
}
func (m *ClientState) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ClientState) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ClientState.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *ClientState) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ClientState.Merge(m, src)
}
func (m *ClientState) XXX_Size() int {
	return m.Size()
}
func (m *ClientState) XXX_DiscardUnknown() {
	xxx_messageInfo_ClientState.DiscardUnknown(m)
}

var xxx_messageInfo_ClientState proto.InternalMessageInfo

func (m *ClientState) GetTrustingPeriod() uint64 {
	if m != nil {
		return m.TrustingPeriod
	}
	return 0
}

func (m *ClientState) GetFrozenHeight() uint64 {
	if m != nil {
		return m.FrozenHeight
	}
	return 0
}

func (m *ClientState) GetMaxClockDrift() uint64 {
	if m != nil {
		return m.MaxClockDrift
	}
	return 0
}

func (m *ClientState) GetLatestHeight() uint64 {
	if m != nil {
		return m.LatestHeight
	}
	return 0
}

func (m *ClientState) GetNetworkSectionHash() []byte {
	if m != nil {
		return m.NetworkSectionHash
	}
	return nil
}

func (m *ClientState) GetValidators() [][]byte {
	if m != nil {
		return m.Validators
	}
	return nil
}

type ConsensusState struct {
	MessageRoot []byte `protobuf:"bytes,1,opt,name=message_root,json=messageRoot,proto3" json:"message_root,omitempty"`
}

func (m *ConsensusState) Reset()         { *m = ConsensusState{} }
func (m *ConsensusState) String() string { return proto.CompactTextString(m) }
func (*ConsensusState) ProtoMessage()    {}
func (*ConsensusState) Descriptor() ([]byte, []int) {
	return fileDescriptor_5ae86e09394aefe7, []int{1}
}
func (m *ConsensusState) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ConsensusState) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ConsensusState.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *ConsensusState) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ConsensusState.Merge(m, src)
}
func (m *ConsensusState) XXX_Size() int {
	return m.Size()
}
func (m *ConsensusState) XXX_DiscardUnknown() {
	xxx_messageInfo_ConsensusState.DiscardUnknown(m)
}

var xxx_messageInfo_ConsensusState proto.InternalMessageInfo

func (m *ConsensusState) GetMessageRoot() []byte {
	if m != nil {
		return m.MessageRoot
	}
	return nil
}

type BlockUpdate struct {
	Header *SignedHeader `protobuf:"bytes,1,opt,name=header,proto3" json:"header,omitempty"`
}

func (m *BlockUpdate) Reset()         { *m = BlockUpdate{} }
func (m *BlockUpdate) String() string { return proto.CompactTextString(m) }
func (*BlockUpdate) ProtoMessage()    {}
func (*BlockUpdate) Descriptor() ([]byte, []int) {
	return fileDescriptor_5ae86e09394aefe7, []int{2}
}
func (m *BlockUpdate) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *BlockUpdate) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_BlockUpdate.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *BlockUpdate) XXX_Merge(src proto.Message) {
	xxx_messageInfo_BlockUpdate.Merge(m, src)
}
func (m *BlockUpdate) XXX_Size() int {
	return m.Size()
}
func (m *BlockUpdate) XXX_DiscardUnknown() {
	xxx_messageInfo_BlockUpdate.DiscardUnknown(m)
}

var xxx_messageInfo_BlockUpdate proto.InternalMessageInfo

func (m *BlockUpdate) GetHeader() *SignedHeader {
	if m != nil {
		return m.Header
	}
	return nil
}

type Misbehaviour struct {
	ClientId string       `protobuf:"bytes,1,opt,name=client_id,json=clientId,proto3" json:"client_id,omitempty"`
	Header_1 *BlockUpdate `protobuf:"bytes,2,opt,name=header_1,json=header1,proto3" json:"header_1,omitempty"`
	Header_2 *BlockUpdate `protobuf:"bytes,3,opt,name=header_2,json=header2,proto3" json:"header_2,omitempty"`
}

func (m *Misbehaviour) Reset()         { *m = Misbehaviour{} }
func (m *Misbehaviour) String() string { return proto.CompactTextString(m) }
func (*Misbehaviour) ProtoMessage()    {}
func (*Misbehaviour) Descriptor() ([]byte, []int) {
	return fileDescriptor_5ae86e09394aefe7, []int{3}
}
func (m *Misbehaviour) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *Misbehaviour) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_Misbehaviour.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *Misbehaviour) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Misbehaviour.Merge(m, src)
}
func (m *Misbehaviour) XXX_Size() int {
	return m.Size()
}
func (m *Misbehaviour) XXX_DiscardUnknown() {
	xxx_messageInfo_Misbehaviour.DiscardUnknown(m)
}

var xxx_messageInfo_Misbehaviour proto.InternalMessageInfo

func (m *Misbehaviour) GetClientId() string {
	if m != nil {
		return m.ClientId
	}
	return ""
}

func (m *Misbehaviour) GetHeader_1() *BlockUpdate {
	if m != nil {
		return m.Header_1
	}
	return nil
}

func (m *Misbehaviour) GetHeader_2() *BlockUpdate {
	if m != nil {
		return m.Header_2
	}
	return nil
}

func init() {
	proto.RegisterType((*ClientState)(nil), "icon.lightclient.v1.ClientState")
	proto.RegisterType((*ConsensusState)(nil), "icon.lightclient.v1.ConsensusState")
	proto.RegisterType((*BlockUpdate)(nil), "icon.lightclient.v1.BlockUpdate")
	proto.RegisterType((*Misbehaviour)(nil), "icon.lightclient.v1.Misbehaviour")
}

func init() { proto.RegisterFile("icon/lightclient/v1/light.proto", fileDescriptor_5ae86e09394aefe7) }

var fileDescriptor_5ae86e09394aefe7 = []byte{
	// 518 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x94, 0x92, 0xc1, 0x6e, 0xd3, 0x30,
	0x1c, 0xc6, 0xe7, 0x76, 0x94, 0xcd, 0xcd, 0x36, 0x29, 0x03, 0x51, 0x98, 0xc8, 0x4a, 0x91, 0xa0,
	0xa7, 0x94, 0xb4, 0xb7, 0xee, 0x96, 0x22, 0xd1, 0x4a, 0x9d, 0x54, 0xa5, 0xa2, 0x42, 0xa8, 0x52,
	0xe4, 0x26, 0x5e, 0x62, 0x2d, 0xb1, 0x2b, 0xdb, 0x0d, 0x83, 0xa7, 0xe0, 0x19, 0x90, 0xb8, 0xf0,
	0x24, 0x68, 0xa7, 0x1d, 0x39, 0xa2, 0xf6, 0xc6, 0x23, 0x70, 0x42, 0xb6, 0x3b, 0x29, 0x87, 0x5e,
	0xb8, 0x54, 0xcd, 0x2f, 0xbf, 0xef, 0x8b, 0xfd, 0xb7, 0xe1, 0x39, 0x89, 0x18, 0xed, 0x64, 0x24,
	0x49, 0x65, 0x94, 0x11, 0x4c, 0x65, 0xa7, 0xf0, 0xcc, 0xa3, 0xbb, 0xe4, 0x4c, 0x32, 0xfb, 0x54,
	0x09, 0x6e, 0x49, 0x70, 0x0b, 0xef, 0xd9, 0x53, 0x9d, 0x92, 0x9f, 0x97, 0x58, 0x28, 0x5f, 0xff,
	0x31, 0x7e, 0xeb, 0x2f, 0x80, 0xf5, 0x81, 0x16, 0xa7, 0x12, 0x49, 0x6c, 0xbf, 0x86, 0x27, 0x92,
	0xaf, 0x84, 0x24, 0x34, 0x09, 0x97, 0x98, 0x13, 0x16, 0x37, 0x40, 0x13, 0xb4, 0xf7, 0x83, 0xe3,
	0x7b, 0x3c, 0xd1, 0xd4, 0x7e, 0x09, 0x8f, 0xae, 0x38, 0xfb, 0x82, 0x69, 0x98, 0x62, 0xf5, 0xb5,
	0x46, 0x45, 0x6b, 0x96, 0x81, 0x43, 0xcd, 0xec, 0x57, 0xf0, 0x24, 0x47, 0x37, 0x61, 0x94, 0xb1,
	0xe8, 0x3a, 0x8c, 0x39, 0xb9, 0x92, 0x8d, 0xaa, 0xd6, 0x8e, 0x72, 0x74, 0x33, 0x50, 0xf4, 0xad,
	0x82, 0xaa, 0x2c, 0x43, 0x12, 0x0b, 0x79, 0x5f, 0xb6, 0x6f, 0xca, 0x0c, 0xdc, 0x96, 0xbd, 0x81,
	0x8f, 0x28, 0x96, 0x9f, 0x18, 0xbf, 0x0e, 0x05, 0x8e, 0x24, 0x61, 0x34, 0x4c, 0x91, 0x48, 0x1b,
	0x0f, 0x9a, 0xa0, 0x6d, 0x05, 0xf6, 0xf6, 0xdd, 0xd4, 0xbc, 0x1a, 0x22, 0x91, 0xda, 0x0e, 0x84,
	0x05, 0xca, 0x48, 0x8c, 0x24, 0xe3, 0xa2, 0x51, 0x6b, 0x56, 0xdb, 0x56, 0x50, 0x22, 0xad, 0x1e,
	0x3c, 0x1e, 0x30, 0x2a, 0x30, 0x15, 0x2b, 0x61, 0xb6, 0xff, 0x02, 0x5a, 0x39, 0x16, 0x02, 0x25,
	0x38, 0xe4, 0x8c, 0x49, 0xbd, 0x77, 0x2b, 0xa8, 0x6f, 0x59, 0xc0, 0x98, 0x6c, 0xf9, 0xb0, 0xee,
	0xab, 0x95, 0xbf, 0x5f, 0xc6, 0x2a, 0xd1, 0x83, 0xb5, 0x14, 0xa3, 0x18, 0x73, 0xed, 0xd6, 0xbb,
	0x67, 0xae, 0x3e, 0x01, 0x33, 0xe3, 0xc2, 0x73, 0xa7, 0x24, 0xa1, 0x38, 0x1e, 0x6a, 0x25, 0xd8,
	0xaa, 0xad, 0xef, 0x00, 0x5a, 0x97, 0x44, 0x2c, 0x70, 0x8a, 0x0a, 0xc2, 0x56, 0xdc, 0x3e, 0x83,
	0x87, 0xe6, 0xb8, 0x42, 0x62, 0x06, 0x7e, 0x18, 0x1c, 0x18, 0x30, 0x8a, 0xed, 0x0b, 0x78, 0x60,
	0x72, 0xa1, 0xa7, 0xa7, 0x5c, 0xef, 0x36, 0xdd, 0x1d, 0xc7, 0xec, 0x96, 0x96, 0x15, 0x3c, 0x34,
	0x09, 0xaf, 0x14, 0xee, 0xea, 0xd9, 0xff, 0x47, 0xb8, 0xeb, 0xdf, 0x82, 0x9f, 0x6b, 0x07, 0xdc,
	0xad, 0x1d, 0xf0, 0x7b, 0xed, 0x80, 0xaf, 0x1b, 0x67, 0xef, 0x6e, 0xe3, 0xec, 0xfd, 0xda, 0x38,
	0x7b, 0xf0, 0x49, 0xc4, 0xf2, 0x5d, 0x45, 0x3e, 0x1c, 0xab, 0xe7, 0x89, 0xba, 0x5d, 0x13, 0xf0,
	0xf1, 0x79, 0x46, 0x16, 0x1c, 0x71, 0x82, 0x45, 0x27, 0x61, 0x9d, 0x88, 0xe5, 0x39, 0xa3, 0x1d,
	0x15, 0xbb, 0x50, 0x3f, 0xdf, 0x2a, 0xd5, 0xd1, 0xf8, 0xc3, 0x8f, 0xca, 0xe9, 0x48, 0x15, 0x8d,
	0x4b, 0x45, 0x33, 0xef, 0xd6, 0xd0, 0x79, 0x89, 0xce, 0x67, 0xde, 0xba, 0x72, 0xbe, 0x83, 0xce,
	0xdf, 0x4d, 0xfc, 0x4b, 0x2c, 0x51, 0x8c, 0x24, 0xfa, 0x53, 0x79, 0xac, 0x8c, 0x7e, 0xbf, 0xa4,
	0xf4, 0xfb, 0x33, 0x6f, 0x51, 0xd3, 0x37, 0xbe, 0xf7, 0x2f, 0x00, 0x00, 0xff, 0xff, 0xf8, 0x65,
	0x76, 0xbb, 0x44, 0x03, 0x00, 0x00,
}

func (m *ClientState) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ClientState) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *ClientState) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Validators) > 0 {
		for iNdEx := len(m.Validators) - 1; iNdEx >= 0; iNdEx-- {
			i -= len(m.Validators[iNdEx])
			copy(dAtA[i:], m.Validators[iNdEx])
			i = encodeVarintLight(dAtA, i, uint64(len(m.Validators[iNdEx])))
			i--
			dAtA[i] = 0x32
		}
	}
	if len(m.NetworkSectionHash) > 0 {
		i -= len(m.NetworkSectionHash)
		copy(dAtA[i:], m.NetworkSectionHash)
		i = encodeVarintLight(dAtA, i, uint64(len(m.NetworkSectionHash)))
		i--
		dAtA[i] = 0x2a
	}
	if m.LatestHeight != 0 {
		i = encodeVarintLight(dAtA, i, uint64(m.LatestHeight))
		i--
		dAtA[i] = 0x20
	}
	if m.MaxClockDrift != 0 {
		i = encodeVarintLight(dAtA, i, uint64(m.MaxClockDrift))
		i--
		dAtA[i] = 0x18
	}
	if m.FrozenHeight != 0 {
		i = encodeVarintLight(dAtA, i, uint64(m.FrozenHeight))
		i--
		dAtA[i] = 0x10
	}
	if m.TrustingPeriod != 0 {
		i = encodeVarintLight(dAtA, i, uint64(m.TrustingPeriod))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *ConsensusState) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ConsensusState) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *ConsensusState) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.MessageRoot) > 0 {
		i -= len(m.MessageRoot)
		copy(dAtA[i:], m.MessageRoot)
		i = encodeVarintLight(dAtA, i, uint64(len(m.MessageRoot)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *BlockUpdate) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *BlockUpdate) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *BlockUpdate) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.Header != nil {
		{
			size, err := m.Header.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintLight(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *Misbehaviour) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Misbehaviour) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Misbehaviour) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.Header_2 != nil {
		{
			size, err := m.Header_2.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintLight(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x1a
	}
	if m.Header_1 != nil {
		{
			size, err := m.Header_1.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintLight(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x12
	}
	if len(m.ClientId) > 0 {
		i -= len(m.ClientId)
		copy(dAtA[i:], m.ClientId)
		i = encodeVarintLight(dAtA, i, uint64(len(m.ClientId)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func encodeVarintLight(dAtA []byte, offset int, v uint64) int {
	offset -= sovLight(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *ClientState) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.TrustingPeriod != 0 {
		n += 1 + sovLight(uint64(m.TrustingPeriod))
	}
	if m.FrozenHeight != 0 {
		n += 1 + sovLight(uint64(m.FrozenHeight))
	}
	if m.MaxClockDrift != 0 {
		n += 1 + sovLight(uint64(m.MaxClockDrift))
	}
	if m.LatestHeight != 0 {
		n += 1 + sovLight(uint64(m.LatestHeight))
	}
	l = len(m.NetworkSectionHash)
	if l > 0 {
		n += 1 + l + sovLight(uint64(l))
	}
	if len(m.Validators) > 0 {
		for _, b := range m.Validators {
			l = len(b)
			n += 1 + l + sovLight(uint64(l))
		}
	}
	return n
}

func (m *ConsensusState) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.MessageRoot)
	if l > 0 {
		n += 1 + l + sovLight(uint64(l))
	}
	return n
}

func (m *BlockUpdate) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Header != nil {
		l = m.Header.Size()
		n += 1 + l + sovLight(uint64(l))
	}
	return n
}

func (m *Misbehaviour) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.ClientId)
	if l > 0 {
		n += 1 + l + sovLight(uint64(l))
	}
	if m.Header_1 != nil {
		l = m.Header_1.Size()
		n += 1 + l + sovLight(uint64(l))
	}
	if m.Header_2 != nil {
		l = m.Header_2.Size()
		n += 1 + l + sovLight(uint64(l))
	}
	return n
}

func sovLight(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozLight(x uint64) (n int) {
	return sovLight(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *ClientState) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowLight
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ClientState: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ClientState: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TrustingPeriod", wireType)
			}
			m.TrustingPeriod = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TrustingPeriod |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field FrozenHeight", wireType)
			}
			m.FrozenHeight = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.FrozenHeight |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field MaxClockDrift", wireType)
			}
			m.MaxClockDrift = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.MaxClockDrift |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 4:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field LatestHeight", wireType)
			}
			m.LatestHeight = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.LatestHeight |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 5:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field NetworkSectionHash", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthLight
			}
			postIndex := iNdEx + byteLen
			if postIndex < 0 {
				return ErrInvalidLengthLight
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.NetworkSectionHash = append(m.NetworkSectionHash[:0], dAtA[iNdEx:postIndex]...)
			if m.NetworkSectionHash == nil {
				m.NetworkSectionHash = []byte{}
			}
			iNdEx = postIndex
		case 6:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Validators", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthLight
			}
			postIndex := iNdEx + byteLen
			if postIndex < 0 {
				return ErrInvalidLengthLight
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Validators = append(m.Validators, make([]byte, postIndex-iNdEx))
			copy(m.Validators[len(m.Validators)-1], dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipLight(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthLight
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *ConsensusState) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowLight
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ConsensusState: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ConsensusState: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field MessageRoot", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthLight
			}
			postIndex := iNdEx + byteLen
			if postIndex < 0 {
				return ErrInvalidLengthLight
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.MessageRoot = append(m.MessageRoot[:0], dAtA[iNdEx:postIndex]...)
			if m.MessageRoot == nil {
				m.MessageRoot = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipLight(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthLight
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *BlockUpdate) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowLight
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: BlockUpdate: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: BlockUpdate: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Header", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthLight
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthLight
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Header == nil {
				m.Header = &SignedHeader{}
			}
			if err := m.Header.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipLight(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthLight
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *Misbehaviour) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowLight
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: Misbehaviour: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Misbehaviour: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ClientId", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthLight
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthLight
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.ClientId = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Header_1", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthLight
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthLight
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Header_1 == nil {
				m.Header_1 = &BlockUpdate{}
			}
			if err := m.Header_1.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Header_2", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowLight
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthLight
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthLight
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Header_2 == nil {
				m.Header_2 = &BlockUpdate{}
			}
			if err := m.Header_2.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipLight(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthLight
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipLight(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowLight
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowLight
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowLight
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthLight
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupLight
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthLight
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthLight        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowLight          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupLight = fmt.Errorf("proto: unexpected end of group")
)
