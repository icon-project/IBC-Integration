
use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn verify_connection_delay_passed(
        &self,
        deps: DepsMut,
        packet_proof_height: Height,
        connection_end: ConnectionEnd,
    ) -> Result<(), ContractError> {
        // Fetch the current host chain time and height.
        let current_host_time = self.host_timestamp(deps.storage)?;
        let current_host_height = self.host_height()?;

        // Fetch the latest time and height that the counterparty client was updated on the host chain.
        let client_id = connection_end.client_id();
        let last_client_update_time = self.client_update_time(client_id, &packet_proof_height)?;
        let last_client_update_height =
            self.client_update_height(client_id, &packet_proof_height)?;

        // Fetch the connection delay time and height periods.
        let conn_delay_time_period = connection_end.delay_period();
        let conn_delay_height_period = self.block_delay(&conn_delay_time_period);

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

    pub fn block_delay(&self, delay_period_time: &Duration) -> u64 {
        calculate_block_delay(delay_period_time, &self.max_expected_time_per_block())
    }
}
