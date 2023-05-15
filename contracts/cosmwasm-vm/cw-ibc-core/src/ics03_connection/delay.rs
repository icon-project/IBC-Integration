use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This method is verifying that the connection delay period has passed for a given connection on
    /// the host chain. It takes in the current state of the connection, including the latest time and
    /// height that the counterparty client was updated on the host chain, and the connection delay time
    /// and height periods. It then calculates the earliest valid time and height for the connection and
    /// checks if the current host time and height have surpassed those values. If they have not, it
    /// returns an error indicating that not enough time or blocks have elapsed. If they have, it returns
    /// Ok(()) indicating that the connection delay period has passed.
    pub fn verify_connection_delay_passed(
        &self,
        store: &dyn Storage,
        packet_proof_height: Height,
        connection_end: ConnectionEnd,
    ) -> Result<(), ContractError> {
        // Fetch the current host chain time and height.
        let current_host_time = self.host_timestamp(store)?;
        let current_host_height = self.host_height()?;

        // Fetch the latest time and height that the counterparty client was updated on the host chain.
        let client_id = connection_end.client_id();
        let last_client_update_time = self.client_update_time(client_id, &packet_proof_height)?;
        let last_client_update_height =
            self.client_update_height(client_id, &packet_proof_height)?;

        // Fetch the connection delay time and height periods.
        let conn_delay_time_period = connection_end.delay_period();
        let conn_delay_height_period = self.calc_block_delay(&conn_delay_time_period);

        let earliest_valid_time =
            (last_client_update_time + conn_delay_time_period).map_err(|e| {
                ContractError::IbcConnectionError {
                    error: ConnectionError::TimestampOverflow(e),
                }
            })?;
        if current_host_time < earliest_valid_time {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::NotEnoughTimeElapsed {
                    current_host_time,
                    earliest_valid_time,
                },
            })?;
        }

        let earliest_valid_height = last_client_update_height.add(conn_delay_height_period);
        if current_host_height < earliest_valid_height {
            return Err(ContractError::IbcConnectionError {
                error: ConnectionError::NotEnoughBlocksElapsed {
                    current_host_height,
                    earliest_valid_height,
                },
            })?;
        }

        Ok(())
    }
}
