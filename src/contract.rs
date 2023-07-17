#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, SubMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, CustomMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CustomMsg>, ContractError> {
    match msg {
        ExecuteMsg::Test { type_url, value } => test(deps, type_url, value),
    }
}

pub fn test(_deps: DepsMut, type_url: String, value: String) -> Result<Response<CustomMsg>, ContractError> {
    let msg = CosmosMsg::Stargate { 
        type_url: type_url, 
        value: Binary::from(value.as_bytes()),
    };
    Ok(Response::new()
        .add_attribute("method", "test")
        .add_submessage(SubMsg::new(msg)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary("ok")
}
