package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/cosmos/cosmos-sdk/types"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

var ibcConfigSetup = `{"ibc_config":{"msg":{"open_ack":{"channel":{"endpoint":{"port_id":"our-port","channel_id":"channel-1"},"counterparty_endpoint":{"port_id":"their-port","channel_id":"channel-3"},"order":"ORDER_UNORDERED","version":"xcall-1","connection_id":"newconnection"},"counterparty_version":"xcall-1"}}}}`

var packetReceiveData = `{"ibc_packet_receive":{"msg":{"packet":{"data":"+FfBALhT+FHGhWFsaWNluEJhcmNod2F5MW45emhtaDY0YTJ2MmY5cDlwajh4MHU1YzQ5bHV3bWZrM2pxdmVqeGs5dHpxbXN1ajlsOXNhdnhydTMBgIMBAgM=","src":{"port_id":"our-port","channel_id":"channel-1"},"dest":{"port_id":"their-port","channel_id":"channel-3"},"sequence":0,"timeout":{"block":{"revision":0,"height":0},"timestamp":null}},"relayer":"relay"}}}`

var args = `{"send_call_message":{"to":"hjhbd","data":[1,2,3],"rollback":[3,4,5]}}`
var packetExceedLimit = `{"send_call_message":{"to":"hjhbd","data":[72, 117, 121, 107, 99, 66, 84, 88, 102, 112, 100, 100, 100, 100, 100, 77, 109, 98, 119, 87, 120, 57, 67, 79, 90, 87, 98, 117, 77, 107, 101, 99, 110, 77, 84, 71, 73, 53, 52, 111, 88, 70, 66, 115, 83, 70, 79, 121, 112, 86, 72, 117, 105, 66, 84, 50, 101, 103, 104, 48, 100, 114, 99, 82, 65, 83, 52, 119, 81, 121, 71, 120, 79, 67, 71, 104, 76, 56, 109, 66, 87, 84, 116, 116, 69, 79, 71, 56, 56, 107, 117, 118, 68, 99, 70, 53, 82, 53, 79, 109, 104, 66, 97, 49, 98, 101, 111, 52, 54, 105, 57, 73, 119, 68, 53, 54, 79, 113, 104, 112, 122, 67, 79, 86, 120, 74, 113, 70, 56, 55, 102, 104, 99, 116, 65, 121, 109, 104, 109, 77, 83, 66, 87, 65, 57, 53, 103, 67, 110, 78, 80, 52, 53, 73, 102, 53, 70, 102, 70, 116, 82, 73, 115, 105, 85, 57, 102, 107, 119, 113, 89, 80, 82, 75, 67, 112, 106, 116, 119, 115, 70, 99, 89, 74, 83, 66, 51, 102, 109, 65, 66, 68, 102, 115, 105, 81, 66, 84, 51, 114, 67, 106, 118, 87, 121, 98, 122, 114, 100, 78, 51, 78, 111, 83, 52, 86, 72, 84, 51, 115, 75, 117, 122, 86, 101, 79, 84, 78, 83, 72, 68, 71, 90, 97, 122, 116, 69, 112, 82, 113, 66, 66, 88, 49, 78, 103, 78, 77, 100, 107, 121, 54, 51, 120, 102, 67, 99, 115, 108, 66, 117, 106, 114, 121, 90, 73, 98, 84, 51, 120, 70, 79, 88, 81, 84, 122, 104, 109, 67, 113, 121, 112, 113, 67, 66, 115, 102, 69, 50, 73, 75, 98, 82, 112, 90, 52, 122, 106, 74, 106, 82, 72, 104, 75, 57, 101, 50, 72, 50, 69, 84, 104, 107, 48, 104, 117, 80, 55, 81, 86, 75, 107, 72, 74, 50, 85, 69, 67, 121, 106, 56, 81, 97, 104, 113, 118, 113, 119, 116, 75, 51, 81, 79, 86, 56, 80, 78, 49, 108, 81, 109, 97, 76, 86, 56, 103, 116, 75, 117, 66, 69, 81, 97, 108, 81, 83, 99, 72, 111, 112, 88, 79, 67, 98, 101, 83, 90, 103, 114, 71, 82, 69, 48, 114, 52, 52, 55, 105, 55, 112, 112, 67, 76, 105, 54, 80, 98, 88, 51, 113, 106, 97, 49, 82, 51, 85, 120, 77, 81, 50, 109, 84, 73, 113, 90, 82, 119, 65, 115, 113, 70, 72, 97, 122, 108, 55, 104, 106, 99, 104, 113, 75, 107, 76, 75, 114, 98, 99, 48, 89, 82, 122, 51, 101, 103, 81, 100, 90, 105, 53, 53, 99, 55, 66, 66, 112, 118, 119, 71, 76, 118, 69, 101, 72, 85, 70, 72, 52, 113, 114, 83, 98, 90, 54, 111, 82, 72, 79, 74, 102, 121, 87, 97, 66, 116, 84, 115, 111, 90, 122, 106, 65, 65, 112, 83, 76, 57, 52, 69, 70, 99, 70, 106, 90, 86, 55, 98, 53, 73, 109, 68, 116, 49, 117, 118, 67, 121, 56, 108, 71, 85, 76, 77, 105, 103, 56, 68, 56, 88, 87, 99, 100, 89, 81, 87, 100, 108, 77, 89, 119, 118, 83, 116, 122, 122, 68, 112, 113, 66, 85, 50, 116, 119, 49, 100, 88, 57, 111, 109, 68, 55, 73, 74, 78, 99, 66, 110, 89, 78, 81, 69, 88, 116, 105, 69, 71, 68, 100, 104, 110, 67, 68, 70, 57, 122, 54, 108, 120, 72, 48, 74, 72, 71, 57, 90, 101, 112, 98, 105, 77, 75, 105, 49, 98, 100, 117, 104, 90, 114, 85, 82, 53, 49, 103, 107, 113, 78, 80, 84, 51, 74, 120, 122, 105, 65, 108, 78, 50, 120, 117, 97, 77, 57, 102, 51, 84, 86, 73, 113, 78, 76, 73, 57, 73, 106, 74, 89, 65, 70, 78, 73, 113, 101, 52, 73, 90, 55, 113, 102, 74, 67, 83, 73, 111, 68, 106, 50, 84, 113, 48, 119, 74, 114, 69, 107, 88, 103, 87, 56, 107, 65, 104, 101, 77, 122, 118, 109, 79, 86, 103, 108, 114, 49, 83, 108, 83, 111, 51, 117, 119, 101, 86, 97, 79, 103, 102, 71, 98, 119, 65, 78, 97, 107, 51, 57, 77, 116, 112, 108, 121, 107, 115, 103, 72, 56, 71, 71, 103, 83, 118, 48, 107, 51, 103, 104, 76, 72, 101, 84, 48, 54, 72, 98, 75, 116, 54, 77, 67, 86, 67, 105, 53, 102, 99, 70, 76, 88, 117, 67, 97, 48, 72, 90, 116, 55, 68, 115, 108, 103, 54, 48, 49, 89, 113, 74, 110, 51, 54, 72, 119, 48, 51, 49, 79, 98, 107, 74, 102, 49, 72, 70, 111, 78, 102, 56, 109, 100, 76, 72, 106, 67, 102, 68, 67, 88, 97, 85, 119, 87, 89, 57, 111, 119, 113, 109, 89, 68, 76, 51, 57, 74, 104, 52, 54, 80, 56, 48, 115, 88, 97, 52, 117, 49, 73, 113, 75, 85, 102, 114, 70, 77, 109, 109, 67, 112, 70, 55, 77, 97, 86, 118, 116, 100, 77, 115, 74, 101, 108, 122, 50, 122, 90, 72, 90, 85, 80, 83, 105, 67, 51, 56, 120, 102, 85, 107, 79, 100, 99, 103, 82, 86, 99, 76, 86, 66, 118, 56, 71, 83, 75, 99, 113, 114, 77, 71, 111, 57, 81, 90, 115, 50, 102, 117, 57, 90, 105, 50, 53, 87, 117, 83, 90, 48, 83, 122, 82, 111, 54, 49, 84, 106, 66, 112, 82, 88, 109, 49, 77, 121, 112, 73, 68, 110, 120, 84, 84, 69, 77, 77, 66, 65, 55, 108, 57, 76, 55, 84, 101, 111, 106, 82, 97, 107, 56, 48, 83, 88, 104, 75, 71, 120, 49, 80, 106, 52, 65, 75, 75, 78, 71, 105, 75, 89, 101, 73, 104, 121, 120, 51, 101, 83, 76, 49, 74, 122, 88, 109, 87, 57, 113, 65, 66, 78, 54, 101, 120, 49, 77, 75, 52, 118, 56, 112, 100, 86, 105, 77, 115, 122, 80, 103, 87, 106, 101, 65, 76, 57, 53, 104, 73, 87, 90, 72, 117, 81, 82, 77, 81, 107, 84, 87, 54, 65, 56, 122, 66, 73, 108, 116, 109, 66, 114, 77, 55, 72, 65, 86, 88, 98, 103, 118, 77, 69, 78, 52, 56, 77, 105, 97, 99, 118, 70, 55, 117, 121, 67, 52, 111, 103, 112, 116, 79, 119, 50, 77, 48, 49, 82, 80, 88, 53, 118, 103, 114, 89, 83, 48, 117, 88, 105, 78, 85, 101, 51, 65, 107, 107, 80, 77, 53, 50, 122, 55, 51, 116, 54, 122, 99, 78, 116, 66, 49, 101, 121, 49, 112, 57, 57, 72, 108, 118, 86, 105, 55, 69, 83, 107, 80, 102, 119, 81, 54, 77, 87, 73, 50, 77, 48, 98, 74, 106, 114, 117, 57, 113, 89, 108, 108, 54, 49, 105, 100, 68, 87, 51, 72, 48, 53, 118, 55, 102, 70, 116, 71, 103, 56, 73, 99, 48, 77, 121, 77, 98, 122, 83, 88, 49, 49, 53, 71, 73, 77, 110, 54, 119, 97, 100, 72, 117, 98, 121, 97, 79, 76, 78, 67, 84, 74, 122, 115, 65, 112, 99, 119, 117, 86, 68, 85, 98, 55, 117, 89, 120, 82, 107, 98, 53, 51, 90, 80, 52, 118, 86, 75, 98, 80, 113, 71, 117, 103, 99, 81, 111, 106, 106, 113, 50, 50, 114, 89, 78, 84, 74, 116, 48, 102, 114, 105, 103, 121, 81, 89, 112, 88, 109, 56, 70, 49, 66, 48, 54, 86, 99, 72, 110, 85, 106, 52, 54, 48, 107, 88, 69, 88, 114, 112, 101, 112, 55, 85, 107, 80, 97, 82, 88, 53, 113, 108, 111, 70, 53, 99, 115, 110, 113, 83, 116, 117, 117, 116, 102, 57, 108, 68, 80, 83, 88, 56, 89, 114, 102, 121, 54, 112, 116, 100, 83, 54, 70, 76, 121, 115, 48, 103, 74, 112, 74, 118, 82, 49, 99, 68, 99, 50, 104, 49, 65, 102, 75, 89, 121, 82, 107, 102, 108, 72, 87, 85, 83, 104, 112, 74, 108, 121, 114, 120, 70, 52, 98, 115, 79, 82, 52, 50, 118, 117, 53, 90, 122, 88, 49, 79, 81, 90, 74, 97, 84, 77, 97, 105, 113, 49, 75, 56, 73, 108, 103, 122, 69, 73, 70, 122, 106, 57, 78, 86, 106, 105, 55, 116, 50, 54, 105, 73, 103, 116, 105, 90, 110, 113, 49, 55, 116, 119, 97, 119, 57, 55, 76, 51, 85, 48, 73, 50, 82, 108, 86, 86, 54, 120, 70, 57, 111, 69, 50, 55, 117, 70, 48, 56, 116, 116, 84, 81, 48, 68, 51, 51, 86, 110, 114, 122, 101, 89, 66, 102, 121, 114, 72, 102, 114, 106, 111, 117, 102, 52, 52, 105, 103, 69, 76, 71, 119, 111, 108, 120, 97, 109, 89, 103, 109, 97, 84, 54, 78, 113, 87, 104, 76, 87, 52, 53, 106, 117, 122, 113, 109, 107, 108, 78, 116, 51, 51, 68, 111, 70, 82, 89, 102, 77, 73, 109, 82, 114, 110, 65, 98, 104, 53, 122, 82, 50, 48, 88, 76, 87, 65, 103, 115, 112, 80, 68, 88, 103, 100, 100, 53, 50, 98, 49, 115, 99, 108, 82, 54, 68, 98, 65, 97, 52, 51, 119, 81, 103, 100, 72, 112, 111, 80, 104, 83, 110, 89, 67, 71, 115, 122, 71, 114, 78, 50, 118, 82, 49, 107, 121, 77, 82, 98, 51, 50, 119, 102, 51, 55, 66, 65, 55, 50, 53, 114, 99, 79, 66, 118, 102, 104, 81, 83, 70, 122, 78, 116, 84, 107, 49, 73, 113, 89, 68, 121, 80, 71, 85, 80, 115, 90, 84, 83, 107, 110, 85, 113, 52, 111, 66, 82, 84, 70, 74, 104, 102, 122, 68, 77, 104, 54, 120, 121, 54, 57, 53, 48, 69, 121, 78, 65, 115, 102, 85, 100, 52, 55, 49, 107, 73, 70, 118, 103, 50, 100, 112, 112, 114, 104, 98, 83, 116, 89, 57, 50, 102, 116, 109, 53, 84, 65, 105, 65, 111, 114, 85, 88, 82, 67, 108, 106, 122, 122, 85, 53, 104, 102, 74, 54, 78, 81, 105, 110, 115, 67, 109, 68, 83, 82, 99, 97, 100, 116, 108, 103, 110, 49, 117, 84, 104, 118, 100, 113, 105, 54, 50, 120, 99, 109, 68, 108, 87, 68, 118, 67, 102, 53, 110, 75, 114, 109, 97, 100, 49, 101, 51, 83, 69, 109, 121, 111, 57, 57, 84, 106, 106, 111, 90, 88, 77, 77, 80, 116, 105, 71, 98, 113, 57, 89, 76, 69, 112, 57, 54, 74, 80, 49, 84, 84, 108, 99, 80, 76, 72, 117, 68, 101, 119, 65, 74, 78, 68, 106, 81, 78, 51, 90, 89, 103, 48, 122, 81, 77, 54, 98, 49, 70, 51, 99, 65, 68, 53, 65, 103, 80, 56, 90, 90, 99, 56, 112, 75, 49, 108, 74, 112, 104, 48, 53, 89, 122, 122, 86, 52, 76, 105, 110, 100, 112, 120, 57, 57, 122, 101, 119, 85, 105, 110, 86, 83, 54, 48, 105, 112, 106, 52, 104, 75, 98, 81, 87, 78, 67, 74, 104, 108, 81, 67, 87, 88, 80, 85, 82, 88, 73, 55, 74, 56, 82, 111, 76, 67, 57, 108, 101, 90, 67, 113, 79, 121, 80, 77, 89, 69, 70, 48, 116, 111, 115, 86, 109, 116, 65, 52, 121, 110, 49, 86, 117, 112, 55, 76, 74, 80, 56, 68, 104, 90, 53, 66, 114, 53, 77, 48, 111, 70, 71, 80, 122, 109, 108, 66, 83, 122, 116, 77, 106, 57, 71, 112, 112, 48, 98, 72, 110, 113, 98, 121, 53, 113, 52, 101, 108, 70, 51, 75, 84, 110, 99, 121, 65, 70, 68, 118, 53, 120, 116, 108, 78, 52, 112, 120, 70, 103, 99, 108, 66, 50, 50, 97, 67, 97, 84, 50, 106, 55, 66, 118, 78, 84, 103, 97, 76, 80, 81, 69, 102, 72, 49, 78, 81, 89, 51, 102, 100, 68, 122, 112, 81, 98, 65, 98, 71, 122, 100, 76, 106, 111, 55, 55, 82, 98, 67, 108, 97, 103, 75, 89, 72, 50, 105, 67, 110, 113, 48, 108, 103, 56, 56, 53, 106, 77, 105, 97, 118, 80, 76, 49, 78, 77, 116, 113, 79, 75, 80, 70, 99],"rollback":[3,4,5]}}`
var packetEqualLimit = `{"send_call_message":{"to":"hjhbd","data":[72, 117, 121, 107, 99, 66, 84, 88, 102, 112, 77, 109, 98, 119, 87, 120, 57, 67, 79, 90, 87, 98, 117, 77, 107, 101, 99, 110, 77, 84, 71, 73, 53, 52, 111, 88, 70, 66, 115, 83, 70, 79, 121, 112, 86, 72, 117, 105, 66, 84, 50, 101, 103, 104, 48, 100, 114, 99, 82, 65, 83, 52, 119, 81, 121, 71, 120, 79, 67, 71, 104, 76, 56, 109, 66, 87, 84, 116, 116, 69, 79, 71, 56, 56, 107, 117, 118, 68, 99, 70, 53, 82, 53, 79, 109, 104, 66, 97, 49, 98, 101, 111, 52, 54, 105, 57, 73, 119, 68, 53, 54, 79, 113, 104, 112, 122, 67, 79, 86, 120, 74, 113, 70, 56, 55, 102, 104, 99, 116, 65, 121, 109, 104, 109, 77, 83, 66, 87, 65, 57, 53, 103, 67, 110, 78, 80, 52, 53, 73, 102, 53, 70, 102, 70, 116, 82, 73, 115, 105, 85, 57, 102, 107, 119, 113, 89, 80, 82, 75, 67, 112, 106, 116, 119, 115, 70, 99, 89, 74, 83, 66, 51, 102, 109, 65, 66, 68, 102, 115, 105, 81, 66, 84, 51, 114, 67, 106, 118, 87, 121, 98, 122, 114, 100, 78, 51, 78, 111, 83, 52, 86, 72, 84, 51, 115, 75, 117, 122, 86, 101, 79, 84, 78, 83, 72, 68, 71, 90, 97, 122, 116, 69, 112, 82, 113, 66, 66, 88, 49, 78, 103, 78, 77, 100, 107, 121, 54, 51, 120, 102, 67, 99, 115, 108, 66, 117, 106, 114, 121, 90, 73, 98, 84, 51, 120, 70, 79, 88, 81, 84, 122, 104, 109, 67, 113, 121, 112, 113, 67, 66, 115, 102, 69, 50, 73, 75, 98, 82, 112, 90, 52, 122, 106, 74, 106, 82, 72, 104, 75, 57, 101, 50, 72, 50, 69, 84, 104, 107, 48, 104, 117, 80, 55, 81, 86, 75, 107, 72, 74, 50, 85, 69, 67, 121, 106, 56, 81, 97, 104, 113, 118, 113, 119, 116, 75, 51, 81, 79, 86, 56, 80, 78, 49, 108, 81, 109, 97, 76, 86, 56, 103, 116, 75, 117, 66, 69, 81, 97, 108, 81, 83, 99, 72, 111, 112, 88, 79, 67, 98, 101, 83, 90, 103, 114, 71, 82, 69, 48, 114, 52, 52, 55, 105, 55, 112, 112, 67, 76, 105, 54, 80, 98, 88, 51, 113, 106, 97, 49, 82, 51, 85, 120, 77, 81, 50, 109, 84, 73, 113, 90, 82, 119, 65, 115, 113, 70, 72, 97, 122, 108, 55, 104, 106, 99, 104, 113, 75, 107, 76, 75, 114, 98, 99, 48, 89, 82, 122, 51, 101, 103, 81, 100, 90, 105, 53, 53, 99, 55, 66, 66, 112, 118, 119, 71, 76, 118, 69, 101, 72, 85, 70, 72, 52, 113, 114, 83, 98, 90, 54, 111, 82, 72, 79, 74, 102, 121, 87, 97, 66, 116, 84, 115, 111, 90, 122, 106, 65, 65, 112, 83, 76, 57, 52, 69, 70, 99, 70, 106, 90, 86, 55, 98, 53, 73, 109, 68, 116, 49, 117, 118, 67, 121, 56, 108, 71, 85, 76, 77, 105, 103, 56, 68, 56, 88, 87, 99, 100, 89, 81, 87, 100, 108, 77, 89, 119, 118, 83, 116, 122, 122, 68, 112, 113, 66, 85, 50, 116, 119, 49, 100, 88, 57, 111, 109, 68, 55, 73, 74, 78, 99, 66, 110, 89, 78, 81, 69, 88, 116, 105, 69, 71, 68, 100, 104, 110, 67, 68, 70, 57, 122, 54, 108, 120, 72, 48, 74, 72, 71, 57, 90, 101, 112, 98, 105, 77, 75, 105, 49, 98, 100, 117, 104, 90, 114, 85, 82, 53, 49, 103, 107, 113, 78, 80, 84, 51, 74, 120, 122, 105, 65, 108, 78, 50, 120, 117, 97, 77, 57, 102, 51, 84, 86, 73, 113, 78, 76, 73, 57, 73, 106, 74, 89, 65, 70, 78, 73, 113, 101, 52, 73, 90, 55, 113, 102, 74, 67, 83, 73, 111, 68, 106, 50, 84, 113, 48, 119, 74, 114, 69, 107, 88, 103, 87, 56, 107, 65, 104, 101, 77, 122, 118, 109, 79, 86, 103, 108, 114, 49, 83, 108, 83, 111, 51, 117, 119, 101, 86, 97, 79, 103, 102, 71, 98, 119, 65, 78, 97, 107, 51, 57, 77, 116, 112, 108, 121, 107, 115, 103, 72, 56, 71, 71, 103, 83, 118, 48, 107, 51, 103, 104, 76, 72, 101, 84, 48, 54, 72, 98, 75, 116, 54, 77, 67, 86, 67, 105, 53, 102, 99, 70, 76, 88, 117, 67, 97, 48, 72, 90, 116, 55, 68, 115, 108, 103, 54, 48, 49, 89, 113, 74, 110, 51, 54, 72, 119, 48, 51, 49, 79, 98, 107, 74, 102, 49, 72, 70, 111, 78, 102, 56, 109, 100, 76, 72, 106, 67, 102, 68, 67, 88, 97, 85, 119, 87, 89, 57, 111, 119, 113, 109, 89, 68, 76, 51, 57, 74, 104, 52, 54, 80, 56, 48, 115, 88, 97, 52, 117, 49, 73, 113, 75, 85, 102, 114, 70, 77, 109, 109, 67, 112, 70, 55, 77, 97, 86, 118, 116, 100, 77, 115, 74, 101, 108, 122, 50, 122, 90, 72, 90, 85, 80, 83, 105, 67, 51, 56, 120, 102, 85, 107, 79, 100, 99, 103, 82, 86, 99, 76, 86, 66, 118, 56, 71, 83, 75, 99, 113, 114, 77, 71, 111, 57, 81, 90, 115, 50, 102, 117, 57, 90, 105, 50, 53, 87, 117, 83, 90, 48, 83, 122, 82, 111, 54, 49, 84, 106, 66, 112, 82, 88, 109, 49, 77, 121, 112, 73, 68, 110, 120, 84, 84, 69, 77, 77, 66, 65, 55, 108, 57, 76, 55, 84, 101, 111, 106, 82, 97, 107, 56, 48, 83, 88, 104, 75, 71, 120, 49, 80, 106, 52, 65, 75, 75, 78, 71, 105, 75, 89, 101, 73, 104, 121, 120, 51, 101, 83, 76, 49, 74, 122, 88, 109, 87, 57, 113, 65, 66, 78, 54, 101, 120, 49, 77, 75, 52, 118, 56, 112, 100, 86, 105, 77, 115, 122, 80, 103, 87, 106, 101, 65, 76, 57, 53, 104, 73, 87, 90, 72, 117, 81, 82, 77, 81, 107, 84, 87, 54, 65, 56, 122, 66, 73, 108, 116, 109, 66, 114, 77, 55, 72, 65, 86, 88, 98, 103, 118, 77, 69, 78, 52, 56, 77, 105, 97, 99, 118, 70, 55, 117, 121, 67, 52, 111, 103, 112, 116, 79, 119, 50, 77, 48, 49, 82, 80, 88, 53, 118, 103, 114, 89, 83, 48, 117, 88, 105, 78, 85, 101, 51, 65, 107, 107, 80, 77, 53, 50, 122, 55, 51, 116, 54, 122, 99, 78, 116, 66, 49, 101, 121, 49, 112, 57, 57, 72, 108, 118, 86, 105, 55, 69, 83, 107, 80, 102, 119, 81, 54, 77, 87, 73, 50, 77, 48, 98, 74, 106, 114, 117, 57, 113, 89, 108, 108, 54, 49, 105, 100, 68, 87, 51, 72, 48, 53, 118, 55, 102, 70, 116, 71, 103, 56, 73, 99, 48, 77, 121, 77, 98, 122, 83, 88, 49, 49, 53, 71, 73, 77, 110, 54, 119, 97, 100, 72, 117, 98, 121, 97, 79, 76, 78, 67, 84, 74, 122, 115, 65, 112, 99, 119, 117, 86, 68, 85, 98, 55, 117, 89, 120, 82, 107, 98, 53, 51, 90, 80, 52, 118, 86, 75, 98, 80, 113, 71, 117, 103, 99, 81, 111, 106, 106, 113, 50, 50, 114, 89, 78, 84, 74, 116, 48, 102, 114, 105, 103, 121, 81, 89, 112, 88, 109, 56, 70, 49, 66, 48, 54, 86, 99, 72, 110, 85, 106, 52, 54, 48, 107, 88, 69, 88, 114, 112, 101, 112, 55, 85, 107, 80, 97, 82, 88, 53, 113, 108, 111, 70, 53, 99, 115, 110, 113, 83, 116, 117, 117, 116, 102, 57, 108, 68, 80, 83, 88, 56, 89, 114, 102, 121, 54, 112, 116, 100, 83, 54, 70, 76, 121, 115, 48, 103, 74, 112, 74, 118, 82, 49, 99, 68, 99, 50, 104, 49, 65, 102, 75, 89, 121, 82, 107, 102, 108, 72, 87, 85, 83, 104, 112, 74, 108, 121, 114, 120, 70, 52, 98, 115, 79, 82, 52, 50, 118, 117, 53, 90, 122, 88, 49, 79, 81, 90, 74, 97, 84, 77, 97, 105, 113, 49, 75, 56, 73, 108, 103, 122, 69, 73, 70, 122, 106, 57, 78, 86, 106, 105, 55, 116, 50, 54, 105, 73, 103, 116, 105, 90, 110, 113, 49, 55, 116, 119, 97, 119, 57, 55, 76, 51, 85, 48, 73, 50, 82, 108, 86, 86, 54, 120, 70, 57, 111, 69, 50, 55, 117, 70, 48, 56, 116, 116, 84, 81, 48, 68, 51, 51, 86, 110, 114, 122, 101, 89, 66, 102, 121, 114, 72, 102, 114, 106, 111, 117, 102, 52, 52, 105, 103, 69, 76, 71, 119, 111, 108, 120, 97, 109, 89, 103, 109, 97, 84, 54, 78, 113, 87, 104, 76, 87, 52, 53, 106, 117, 122, 113, 109, 107, 108, 78, 116, 51, 51, 68, 111, 70, 82, 89, 102, 77, 73, 109, 82, 114, 110, 65, 98, 104, 53, 122, 82, 50, 48, 88, 76, 87, 65, 103, 115, 112, 80, 68, 88, 103, 100, 100, 53, 50, 98, 49, 115, 99, 108, 82, 54, 68, 98, 65, 97, 52, 51, 119, 81, 103, 100, 72, 112, 111, 80, 104, 83, 110, 89, 67, 71, 115, 122, 71, 114, 78, 50, 118, 82, 49, 107, 121, 77, 82, 98, 51, 50, 119, 102, 51, 55, 66, 65, 55, 50, 53, 114, 99, 79, 66, 118, 102, 104, 81, 83, 70, 122, 78, 116, 84, 107, 49, 73, 113, 89, 68, 121, 80, 71, 85, 80, 115, 90, 84, 83, 107, 110, 85, 113, 52, 111, 66, 82, 84, 70, 74, 104, 102, 122, 68, 77, 104, 54, 120, 121, 54, 57, 53, 48, 69, 121, 78, 65, 115, 102, 85, 100, 52, 55, 49, 107, 73, 70, 118, 103, 50, 100, 112, 112, 114, 104, 98, 83, 116, 89, 57, 50, 102, 116, 109, 53, 84, 65, 105, 65, 111, 114, 85, 88, 82, 67, 108, 106, 122, 122, 85, 53, 104, 102, 74, 54, 78, 81, 105, 110, 115, 67, 109, 68, 83, 82, 99, 97, 100, 116, 108, 103, 110, 49, 117, 84, 104, 118, 100, 113, 105, 54, 50, 120, 99, 109, 68, 108, 87, 68, 118, 67, 102, 53, 110, 75, 114, 109, 97, 100, 49, 101, 51, 83, 69, 109, 121, 111, 57, 57, 84, 106, 106, 111, 90, 88, 77, 77, 80, 116, 105, 71, 98, 113, 57, 89, 76, 69, 112, 57, 54, 74, 80, 49, 84, 84, 108, 99, 80, 76, 72, 117, 68, 101, 119, 65, 74, 78, 68, 106, 81, 78, 51, 90, 89, 103, 48, 122, 81, 77, 54, 98, 49, 70, 51, 99, 65, 68, 53, 65, 103, 80, 56, 90, 90, 99, 56, 112, 75, 49, 108, 74, 112, 104, 48, 53, 89, 122, 122, 86, 52, 76, 105, 110, 100, 112, 120, 57, 57, 122, 101, 119, 85, 105, 110, 86, 83, 54, 48, 105, 112, 106, 52, 104, 75, 98, 81, 87, 78, 67, 74, 104, 108, 81, 67, 87, 88, 80, 85, 82, 88, 73, 55, 74, 56, 82, 111, 76, 67, 57, 108, 101, 90, 67, 113, 79, 121, 80, 77, 89, 69, 70, 48, 116, 111, 115, 86, 109, 116, 65, 52, 121, 110, 49, 86, 117, 112, 55, 76, 74, 80, 56, 68, 104, 90, 53, 66, 114, 53, 77, 48, 111, 70, 71, 80, 122, 109, 108, 66, 83, 122, 116, 77, 106, 57, 71, 112, 112, 48, 98, 72, 110, 113, 98, 121, 53, 113, 52, 101, 108, 70, 51, 75, 84, 110, 99, 121, 65, 70, 68, 118, 53, 120, 116, 108, 78, 52, 112, 120, 70, 103, 99, 108, 66, 50, 50, 97, 67, 97, 84, 50, 106, 55, 66, 118, 78, 84, 103, 97, 76, 80, 81, 69, 102, 72, 49, 78, 81, 89, 51, 102, 100, 68, 122, 112, 81, 98, 65, 98, 71, 122, 100, 76, 106, 111, 55, 55, 82, 98, 67, 108, 97, 103, 75, 89, 72, 50, 105, 67, 110, 113, 48, 108, 103, 56, 56, 53, 106, 77, 105, 97, 118, 80, 76, 49, 78, 77, 116, 113, 79, 75, 80, 70, 99],"rollback":[3,4,5]}}`
var rollbackExceedLimit = ``
var rollbackEqualLimit = ``
var rollbackLessLimit = ``
var incorrecrRequestID = `{"execute_call":{"request_id":"6"}}`
var correcrRequestID = `{"execute_call":{"request_id":"1"}}`
var nullRequestID = `{"execute_call":{"request_id":""}}`
var correcrSequenceID = `{"execute_rollback":{"sequence_no":"1"}}`

func (c *CosmosLocalnet) GetQueryParam(method string) Query {
	var queryMsg Query
	if strings.Contains(method, "admin") {
		queryMsg = Query{GetAdmin: &GetAdmin{}}
	} else if strings.Contains(method, "fee") {
		queryMsg = Query{GetProtocolFee: &GetProtocolFee{}}
	}
	return queryMsg
}

func (c *CosmosLocalnet) GetExecuteParam(ctx context.Context, methodName, param string) (context.Context, string, error) {
	if strings.Contains(methodName, "set_admin") {
		return c.SetAdminParams(ctx, param)
	} else if strings.Contains(methodName, "update_admin") {
		return c.UpdateAdminParams(ctx, param)
	} else if strings.Contains(methodName, "remove_admin") {
		originalJSON := `{"remove_admin":{}}`
		return ctx, string(originalJSON), nil
	} else if strings.Contains(methodName, "ibc_config") {
		packetData := ibcConfigSetup
		return ctx, string(packetData), nil
	} else if strings.Contains(methodName, "ibc_packet_receive") {
		packetData := packetReceiveData
		return ctx, string(packetData), nil
	} else if strings.Contains(methodName, "send_call_message") {
		return ctx, sendCallData(ctx, param), nil
	} else if strings.Contains(methodName, "execute_call") {
		return ctx, executeCallData(param), nil
	} else if strings.Contains(methodName, "execute_rollback") {
		return ctx, correcrSequenceID, nil
	}
	return ctx, "", nil
}

func sendCallData(ctx context.Context, param string) string {
	sendCall := ""
	if param == "data size greater" {
		sendCall = packetExceedLimit
	} else if param == "data size eqauls" {
		sendCall = packetEqualLimit
	} else if param == "data size less" {
		ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
		dappAddr := ctxValue.ContractAddress["dapp"]
		fmt.Println(args)
		str := fmt.Sprintf(`{"send_call_message":{"to":"%s","data":[1,2,3],"rollback":[3,4,5]}}`, dappAddr)
		fmt.Println(str)
		return str
	} else if param == "rollback size greater" {
		sendCall = rollbackExceedLimit
	} else if param == "rollback size eqauls" {
		sendCall = rollbackEqualLimit
	} else if param == "rollback size less" {
		sendCall = rollbackLessLimit
	} else {
		sendCall = args
	}
	return sendCall
}

func executeCallData(param string) string {
	executeCall := ""
	if param == "incorrect" {
		executeCall = incorrecrRequestID
	} else if param == "correct" {
		executeCall = correcrRequestID
	} else if param == "null" {
		executeCall = nullRequestID
	} else {
		executeCall = correcrRequestID
	}
	return executeCall
}

func (c *CosmosLocalnet) GetAndFundTestUser(
	ctx context.Context,
	keyNamePrefix string,
	amount int64,
	chain ibc.Chain,
) (keyName string, address string, err error) {
	// Check if the address for the given key is already created
	addr, err := c.CosmosChain.GetAddress(ctx, keyNamePrefix)
	adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
	if err != nil {
		chainCfg := c.CosmosChain.Config()
		user, err := chain.BuildWallet(ctx, keyNamePrefix, "")
		if err != nil {
			return "", "", fmt.Errorf("failed to get source user wallet: %w", err)
		}

		err = chain.SendFunds(ctx, chains.FaucetAccountKeyName, ibc.WalletAmount{
			Address: user.FormattedAddress(),
			Amount:  amount,
			Denom:   chainCfg.Denom,
		})

		if err != nil {
			return "", "", fmt.Errorf("failed to get funds from faucet: %w", err)
		}
		fmt.Printf("Address of %s is : %s \n", user.KeyName(), user.FormattedAddress())
		return user.KeyName(), user.FormattedAddress(), nil
	} else {
		return keyNamePrefix, adminAddr, err
	}
}

func (c *CosmosLocalnet) SetAdminParams(ctx context.Context, keyName string) (context.Context, string, error) {
	var admin SetAdmin
	var admins chains.Admins
	originalJSON := `{"set_admin":{"address":""}}`
	json.Unmarshal([]byte(originalJSON), &admin)
	if strings.ToLower(keyName) == "null" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(originalJSON), nil
	} else if strings.ToLower(keyName) == "junk" {
		admin.SetAdmin.Address = "$%#^!(&^%^)"
		updatedJSON, _ := json.Marshal(admin)
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	} else {
		// Check if the given wallet exists if not create a wallet
		addr, err := c.CosmosChain.GetAddress(ctx, keyName)
		if err != nil {
			c.BuildWallets(ctx, keyName)
			addr, _ = c.CosmosChain.GetAddress(ctx, keyName)
		}
		adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
		admin.SetAdmin.Address = adminAddr
		updatedJSON, _ := json.Marshal(admin)
		fmt.Println(string(updatedJSON))
		admins.Admin = map[string]string{
			keyName: adminAddr,
		}
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	}
}

func (c *CosmosLocalnet) UpdateAdminParams(ctx context.Context, keyName string) (context.Context, string, error) {
	var admin UpdateAdmin
	var admins chains.Admins
	originalJSON := `{"update_admin":{"address":""}}`
	json.Unmarshal([]byte(originalJSON), &admin)
	if strings.ToLower(keyName) == "null" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(originalJSON), nil
	} else if strings.ToLower(keyName) == "junk" {
		admin.UpdateAdmin.Address = "$%#^!(&^%^)"
		updatedJSON, _ := json.Marshal(admin)
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	} else {
		// Check if the given wallet exists if not create a wallet
		addr, err := c.CosmosChain.GetAddress(ctx, keyName)
		if err != nil {
			c.BuildWallets(ctx, keyName)
			addr, _ = c.CosmosChain.GetAddress(ctx, keyName)
		}
		adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
		admin.UpdateAdmin.Address = adminAddr
		updatedJSON, _ := json.Marshal(admin)
		fmt.Println(string(updatedJSON))
		admins.Admin = map[string]string{
			keyName: adminAddr,
		}
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	}
}

func (c *CosmosLocalnet) getInitParams(ctx context.Context, contractName string) string {
	var xcallInit XcallInit
	var DappInit DappInit
	if contractName == "xcall" {
		originalJSON := `{"timeout_height":45, "ibc_host":""}`
		json.Unmarshal([]byte(originalJSON), &xcallInit)
		ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
		coreAddr := ctxValue.ContractAddress["ibccore"]
		xcallInit.IbcHost = coreAddr
		updatedInit, _ := json.Marshal(xcallInit)
		fmt.Printf("Init msg for xCall is : %s", string(updatedInit))
		return string(updatedInit)
	} else if contractName == "dapp" {
		originalJSON := `{"address":""}`
		json.Unmarshal([]byte(originalJSON), &DappInit)
		ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
		xcallAddr := ctxValue.ContractAddress["xcall"]
		DappInit.Address = xcallAddr
		updatedInit, _ := json.Marshal(DappInit)
		fmt.Printf("Init msg for Dapp is : %s", string(updatedInit))
		return string(updatedInit)
	}
	return ""
}

func (c *CosmosLocalnet) getExecuteCallParms(ctx context.Context, contractName string) string {
	ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	dappAddress := ctxValue.ContractAddress["dapp"]

	csRequest := CallServiceMessageRequest{
		From:       "somecontractAddress",
		To:         dappAddress,
		SequenceNo: 1,
		Rollback:   false,
		Data:       []byte("helloworld"),
	}

	csRequestEncode, err := csRequest.RlpEncode()

	if err != nil {
		fmt.Println(err)
	}

	csmessageEncoded, err := RlpEncode(csRequestEncode, CallServiceRequest)
	if err != nil {
		fmt.Println(err)
	}

	messageBody := fmt.Sprintf(`{"sequence":10,"source_port":"","source_channel":"","destination_port":"","destination_channel":"","data":%v,"timeout_height":{"revision_number":0,"revision_height":10},"timeout_timestamp":0}`, csmessageEncoded)

	return messageBody
}
