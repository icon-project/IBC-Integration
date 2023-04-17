use ibc::core::ics04_channel::msgs::{ChannelMsg, PacketMsg};

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn lookup_module_by_port(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
    ) -> Result<ModuleId, ContractError> {
        match self
            .ibc_store()
            .port_to_module()
            .may_load(store, port_id.clone())
        {
            Ok(result) => match result {
                Some(port_id) => Ok(port_id),
                None => Err(ContractError::IbcPortError {
                    error: PortError::UnknownPort {
                        port_id: port_id.ibc_port_id().clone(),
                    },
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn store_module_by_port(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        module_id: ModuleId,
    ) -> Result<(), ContractError> {
        Ok(self
            .ibc_store()
            .port_to_module()
            .save(store, port_id, &module_id)?)
    }

    pub fn lookup_module_channel(
        &self,
        store: &mut dyn Storage,
        msg: &ChannelMsg,
    ) -> Result<ModuleId, ContractError> {
        let port_id = match msg {
            ChannelMsg::OpenInit(msg) => &msg.port_id_on_a,
            ChannelMsg::OpenTry(msg) => &msg.port_id_on_b,
            ChannelMsg::OpenAck(msg) => &msg.port_id_on_a,
            ChannelMsg::OpenConfirm(msg) => &msg.port_id_on_b,
            ChannelMsg::CloseInit(msg) => &msg.port_id_on_a,
            ChannelMsg::CloseConfirm(msg) => &msg.port_id_on_b,
        };
        let module_id = self.lookup_module_by_port(store, port_id.clone().into())?;
        Ok(module_id)
    }

    pub fn lookup_module_packet(
        &self,
        store: &mut dyn Storage,
        msg: &PacketMsg,
    ) -> Result<ModuleId, ContractError> {
        let port_id = match msg {
            PacketMsg::Recv(msg) => &msg.packet.port_id_on_b,
            PacketMsg::Ack(msg) => &msg.packet.port_id_on_a,
            PacketMsg::Timeout(msg) => &msg.packet.port_id_on_a,
            PacketMsg::TimeoutOnClose(msg) => &msg.packet.port_id_on_a,
        };
        let module_id = self.lookup_module_by_port(store, port_id.clone().into())?;
        Ok(module_id)
    }
}
