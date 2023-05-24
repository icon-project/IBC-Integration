package tendermint

func (m *TmHeader) ValidateBasic() error { return nil }
func (m *TmHeader) ClientType() string   { return "tendermint" }

func GetPubKeyFromTx(t string, b []byte) isPublicKey_Sum {

	switch t {
	case "secp256k1":
		return &PublicKey_Secp256K1{Secp256K1: b}

	case "ed25519":
		return &PublicKey_Ed25519{Ed25519: b}

	case "sr25519":
		return &PublicKey_Sr25519{Sr25519: b}
	}
	return nil
}
