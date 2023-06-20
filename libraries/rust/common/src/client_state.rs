use std::time::Duration;

use crate::constants::{
    DEFAULT_NETWORK_ID, DEFAULT_NETWORK_TYPE_ID, DEFAULT_SRC_NETWORK_ID, ICON_CLIENT_TYPE,
};
use crate::ibc::core::ics02_client::error::ClientError;

use crate::ibc::core::ics02_client::client_type::ClientType as IbcClientType;
use crate::ibc::Height as IbcHeight;
use crate::{constants::ICON_CLIENT_STATE_TYPE_URL, icon::icon::lightclient::v1::ClientState};
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use prost::Message;

impl ClientState {
    pub fn new(
        trusting_period: u64,
        frozen_height: u64,
        max_clock_drift: u64,
        latest_height: u64,
        network_section_hash: Vec<u8>,
        validators: Vec<Vec<u8>>,
        network_id: u64,
        network_type_id: u64,
        src_network_id: String,
    ) -> Result<Self, ClientError> {
        if max_clock_drift == 0 {
            return Err(ClientError::Other {
                description: "ClientState max-clock-drift must be greater than zero".to_string(),
            });
        }

        Ok(Self {
            trusting_period,
            frozen_height,
            max_clock_drift,
            latest_height,
            network_section_hash,
            validators,
            network_id,
            network_type_id,
            src_network_id,
        })
    }
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = ClientError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use crate::ibc::core::ics02_client::error::ClientError as Error;
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_client_state<B: Buf>(buf: B) -> Result<ClientState, Error> {
            <ClientState as Message>::decode(buf).map_err(ClientError::Decode)
        }

        match raw.type_url.as_str() {
            ICON_CLIENT_STATE_TYPE_URL => decode_client_state(raw.value.deref()),
            _ => Err(ClientError::UnknownClientStateType {
                client_state_type: raw.type_url,
            }),
        }
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Any {
            type_url: ICON_CLIENT_STATE_TYPE_URL.to_string(),
            value: <ClientState as Message>::encode_to_vec(&client_state),
        }
    }
}

pub trait IClientState {
    fn latest_height(&self) -> crate::ibc::Height;
    fn frozen_height(&self) -> Option<crate::ibc::Height>;
    fn expired(&self, elapsed: std::time::Duration) -> bool;
    fn is_frozen(&self) -> bool;
    fn client_type(&self) -> IbcClientType;
}

impl IClientState for ClientState {
    fn latest_height(&self) -> crate::ibc::Height {
        IbcHeight::new(0, self.latest_height).unwrap()
    }

    fn frozen_height(&self) -> Option<crate::ibc::Height> {
        if self.frozen_height == 0 {
            return None;
        }
        Some(IbcHeight::new(0, self.frozen_height).unwrap())
    }

    fn expired(&self, elapsed: std::time::Duration) -> bool {
        let trusting_period = Duration::from_secs(self.trusting_period);
        elapsed.as_secs() > trusting_period.as_secs()
    }

    fn is_frozen(&self) -> bool {
        self.frozen_height > 0
    }

    fn client_type(&self) -> IbcClientType {
        IbcClientType::new(ICON_CLIENT_TYPE.to_string())
    }
}

pub fn get_default_icon_client_state() -> ClientState {
    ClientState {
        network_id: DEFAULT_NETWORK_ID,
        network_type_id: DEFAULT_NETWORK_TYPE_ID,
        src_network_id: DEFAULT_SRC_NETWORK_ID.to_string(),
        ..ClientState::default()
    }
}
