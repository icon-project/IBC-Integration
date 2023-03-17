use super::*;

impl<'a> CwIbcCoreContext<'a> {
    fn connection_end(
        &self,
        conn_id: &ibc::core::ics24_host::identifier::ConnectionId,
    ) -> Result<ibc::core::ics03_connection::connection::ConnectionEnd, ibc::core::ContextError>
    {
        todo!()
    }

    fn store_connection(
        &mut self,
        connection_path: &ibc::core::ics24_host::path::ConnectionPath,
        connection_end: ibc::core::ics03_connection::connection::ConnectionEnd,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn store_connection_to_client(
        &mut self,
        client_connection_path: &ibc::core::ics24_host::path::ClientConnectionPath,
        conn_id: ibc::core::ics24_host::identifier::ConnectionId,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn increase_connection_counter(&mut self) {
        todo!()
    }

    fn connection_counter(&self) -> Result<u64, ibc::core::ContextError> {
        todo!()
    }
}
