#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, from_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{STATE, State};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        owner: _info.sender.clone(),
        price: 0,
    };
    STATE.save(deps.storage, &state)?;

    // TODO: instantiate contract
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    //TODO: execute try_update_price
    match _msg {
        ExecuteMsg::UpdatePrice { price } => try_update_price(_deps, _info, price),
    }

    // Ok(Response::new())
}

fn try_update_price(deps: DepsMut, info: MessageInfo, price: u64) -> Result<Response, ContractError> {

    let current_price = 10; // Luna / Mango

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.price = current_price;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_update_price"))

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::QueryPrice {} => to_binary(&query_price(_deps)?),
    }
    // Err(StdError::generic_err("not implemented"))
}

fn query_price(deps: Deps) -> StdResult<u64> {
    let state = STATE.load(deps.storage)?;
    Ok(state.price)
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { price: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryPrice {}).unwrap();
        let price: u64 = from_binary(&res).unwrap();
        //assert_eq!(price, Err(StdError::generic_err("not implemented")));
        assert_eq!(0, price);
    }

    #[test]
    fn increment() {}
}
