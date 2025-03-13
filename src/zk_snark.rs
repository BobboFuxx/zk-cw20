use cosmwasm_std::{Binary, StdError, StdResult};
use halo2::verify_proof;

pub fn verify_zk_proof(proof: Binary) -> StdResult<bool> {
    verify_proof(&proof).map_err(|_| StdError::generic_err("Invalid zk-SNARK proof"))?;
    Ok(true)
}
