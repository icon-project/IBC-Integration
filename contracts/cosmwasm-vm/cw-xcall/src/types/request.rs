use super::*;
use common::rlp::Nullable;
use cosmwasm_std::Addr;
use cw_xcall_lib::network_address::NetworkAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CallServiceMessageRequest {
    from: NetworkAddress,
    to: Addr,
    sequence_no: u128,
    protocols: Vec<String>,
    rollback: bool,
    data: Nullable<Vec<u8>>,
}

impl CallServiceMessageRequest {
    // TODO : Change to Option of Bytes
    pub fn new(
        from: NetworkAddress,
        to: Addr,
        sequence_no: u128,
        rollback: bool,
        data: Vec<u8>,
        protocols: Vec<String>,
    ) -> Self {
        let data_bytes = match data.is_empty() {
            true => None,
            false => Some(data),
        };
        Self {
            from,
            to,
            sequence_no,
            rollback,
            data: Nullable::new(data_bytes),
            protocols,
        }
    }

    pub fn from(&self) -> &NetworkAddress {
        &self.from
    }

    pub fn to(&self) -> &Addr {
        &self.to
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn rollback(&self) -> bool {
        self.rollback
    }

    pub fn data(&self) -> Result<&[u8], ContractError> {
        Ok(self
            .data
            .get()
            .map_err(|error| ContractError::DecodeFailed {
                error: error.to_string(),
            })?)
    }

    pub fn protocols(&self) -> &Vec<String> {
        &self.protocols
    }
}

impl Encodable for CallServiceMessageRequest {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(6);
        stream.append(&self.from.to_string());
        stream.append(&self.to.to_string());
        stream.append(&self.sequence_no);
        stream.append(&self.rollback);
        stream.append(&self.data);
        stream.begin_list(self.protocols.len());
        for protocol in self.protocols.iter() {
            stream.append(protocol);
        }
    }
}

impl Decodable for CallServiceMessageRequest {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let rlp_protocols = rlp.at(5)?;
        let list: Vec<String> = rlp_protocols.as_list()?;
        let str_from: String = rlp.val_at(0)?;
        let to_str: String = rlp.val_at(1)?;
        Ok(Self {
            from: NetworkAddress::from_str(&str_from)
                .map_err(|_e| rlp::DecoderError::RlpInvalidLength)?,
            to: Addr::unchecked(to_str),
            sequence_no: rlp.val_at(2)?,
            rollback: rlp.val_at(3)?,
            data: rlp.val_at(4)?,
            protocols: list,
        })
    }
}

impl TryFrom<&Vec<u8>> for CallServiceMessageRequest {
    type Error = ContractError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(value as &[u8]);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}

impl TryFrom<&[u8]> for CallServiceMessageRequest {
    type Error = ContractError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(value);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {

    /*
    CSMessageRequest
     from: 0x1.ETH/0xa
     to: cx0000000000000000000000000000000000000102
     sn: 21
     rollback: false
     data: 74657374
     protocol: []
     RLP: F83F8B3078312E4554482F307861AA63783030303030303030303030303030303030303030303030303030303030303030303030303031303215008474657374C0

     CSMessageRequest
     from: 0x1.ETH/0xa
     to: cx0000000000000000000000000000000000000102
     sn: 21
     rollback: false
     data: 74657374
     protocol: [abc, cde, efg]
     RLP: F84B8B3078312E4554482F307861AA63783030303030303030303030303030303030303030303030303030303030303030303030303031303215008474657374CC836162638363646583656667

     CSMessageRequest
     from: 0x1.ETH/0xa
     to: cx0000000000000000000000000000000000000102
     sn: 21
     rollback: true
     data: 74657374
     protocol: [abc, cde, efg]
     RLP: F84B8B3078312E4554482F307861AA63783030303030303030303030303030303030303030303030303030303030303030303030303031303215018474657374CC836162638363646583656667


     */

    use std::str::FromStr;

    use common::rlp;
    use cosmwasm_std::Addr;
    use cw_xcall_lib::network_address::NetworkAddress;

    use super::CallServiceMessageRequest;

    #[test]
    fn test_csmessage_request_encoding() {
        let data = hex::decode("74657374").unwrap();
        let msg = CallServiceMessageRequest::new(
            NetworkAddress::from_str("0x1.ETH/0xa").unwrap(),
            Addr::unchecked("cx0000000000000000000000000000000000000102"),
            21,
            false,
            data.clone(),
            vec![],
        );

        let encoded = rlp::encode(&msg);
        assert_eq!("f83f8b3078312e4554482f307861aa63783030303030303030303030303030303030303030303030303030303030303030303030303031303215008474657374c0",hex::encode(encoded));

        let msg = CallServiceMessageRequest::new(
            NetworkAddress::from_str("0x1.ETH/0xa").unwrap(),
            Addr::unchecked("cx0000000000000000000000000000000000000102"),
            21,
            false,
            data.clone(),
            vec!["abc".to_string(), "cde".to_string(), "efg".to_string()],
        );

        let encoded = rlp::encode(&msg);
        assert_eq!("f84b8b3078312e4554482f307861aa63783030303030303030303030303030303030303030303030303030303030303030303030303031303215008474657374cc836162638363646583656667",hex::encode(encoded));

        let msg = CallServiceMessageRequest::new(
            NetworkAddress::from_str("0x1.ETH/0xa").unwrap(),
            Addr::unchecked("cx0000000000000000000000000000000000000102"),
            21,
            true,
            data,
            vec!["abc".to_string(), "cde".to_string(), "efg".to_string()],
        );

        let encoded = rlp::encode(&msg);
        assert_eq!("f84b8b3078312e4554482f307861aa63783030303030303030303030303030303030303030303030303030303030303030303030303031303215018474657374cc836162638363646583656667",hex::encode(encoded));
    }
}
