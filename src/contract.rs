use cosmwasm_std::{entry_point, Binary, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::zk_snark::verify_zk_proof;
use crate::stealth::generate_stealth_address;
use crate::state::{set_balance, get_balance};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Transfer { recipient_stealth_address, amount, zk_proof } => {
            if verify_zk_proof(zk_proof)? {
                set_balance(deps.storage, &recipient_stealth_address, amount);
                Ok(Response::new().add_attribute("action", "transfer"))
            } else {
                Err(StdError::generic_err("Invalid zk-SNARK proof"))
            }
        }
        ExecuteMsg::GenerateStealthAddress { public_key } => {
            let stealth_address = generate_stealth_address(public_key)?;
            Ok(Response::new()
                .add_attribute("action", "generate_stealth_address")
                .add_attribute("stealth_address", stealth_address.to_base64()))
        }
        _ => Err(StdError::generic_err("Unsupported operation")),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address, viewing_key } => {
            if let Some(balance) = get_balance(deps.storage, &address, &viewing_key) {
                Ok(to_binary(&balance)?)
            } else {
                Err(StdError::generic_err("Invalid viewing key or address"))
            }
        }
        _ => Err(StdError::generic_err("Unsupported query")),
    }
}
