use super::*;

// Makes OpenInitChannel IBC Event
pub fn make_open_init_channel_event(channel_id: &ChannelId, msg: &MsgChannelOpenInit) -> IbcEvent {
    let connection_id = msg.connection_hops_on_a[0].clone();
    let open_init_data = OpenInit::new(
        msg.port_id_on_a.clone(),
        channel_id.ibc_channel_id().clone(),
        msg.port_id_on_b.clone(),
        connection_id,
        msg.version_proposal.clone(),
    );

    IbcEvent::OpenInitChannel(open_init_data)
}

// Makes OpenInitChannel IBC Event
pub fn make_open_try_channel_event(channel_id: &ChannelId, msg: &MsgChannelOpenTry) -> IbcEvent {
    let open_try_data = OpenTry::new(
        msg.port_id_on_b.clone(),
        channel_id.ibc_channel_id().clone(),
        msg.port_id_on_a.clone(),
        msg.chan_id_on_a.clone(),
        msg.connection_hops_on_b[0].clone(),
        msg.version_supported_on_a.clone(),
    );
    IbcEvent::OpenTryChannel(open_try_data)
}




// Makes OpenAckChannel  IBC Event
// pub fn make_open_ack_channel_event(msg: &MsgChannelOpenAck) -> IbcEvent {
//     // port_id_on_a: PortId,
//     // pub chan_id_on_a: ChannelId,
//     // pub chan_id_on_b: ChannelId,
//     // pub version_on_b: Version,
//     // pub proof_chan_end_on_b: CommitmentProofBytes,
//     // pub proof_height_on_b: Height,
//     // pub signer: Signer,

//     let open_try_data = OpenAck::new(
//     );
//     IbcEvent::OpenTryChannel(open_try_data)
// }

// Makes OpenConfirmChannel  IBC Event
// pub fn make_open_confirm_channel_event(
//     msg: &MsgChannelOpenConfirm,
// ) -> IbcEvent {
//     let open_try_data = OpenConfirm::new(
//         msg.port_id_on_b.clone(),
//         channel_id.ibc_channel_id().clone(),
//         msg.port_id_on_a.clone(),
//         msg.chan_id_on_a.clone(),
//         msg.connection_hops_on_b[0].clone(),
//         msg.version_supported_on_a.clone(),
//     );
//     IbcEvent::OpenTryChannel(open_try_data)
// }
