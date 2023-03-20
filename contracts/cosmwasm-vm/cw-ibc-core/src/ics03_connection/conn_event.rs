use super::*;
pub fn event_open_init(conn_id: ConnectionId, msg: MsgConnectionOpenInit) -> IbcEvent {
    let open_init = OpenInit::new(
        conn_id.connection_id().clone(),
        msg.client_id_on_a.clone(),
        msg.counterparty.client_id().clone(),
    );
    IbcEvent::OpenInitConnection(open_init)
}


