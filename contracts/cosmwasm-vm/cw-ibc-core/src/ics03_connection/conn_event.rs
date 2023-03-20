use super::*;
pub fn event_open_init(conn_id: ConnectionId, msg: MsgConnectionOpenInit) -> IbcEvent {
    let open_init = OpenInit::new(
        conn_id.connection_id().clone(),
        msg.client_id_on_a.clone(),
        msg.counterparty.client_id().clone(),
    );
    IbcEvent::OpenInitConnection(open_init)
}

pub fn event_open_try(conn_id: ConnectionId, msg: MsgConnectionOpenTry) -> IbcEvent {
    let open_try = OpenTry::new(
        conn_id.connection_id().clone(),
        msg.client_id_on_b.clone(),
        conn_id.connection_id().clone(),
        msg.counterparty.client_id().clone(),
    );
    IbcEvent::OpenTryConnection(open_try)
}

