#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, SubMsg, Uint128};

use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;

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
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // ExecuteMsg::Test { type_url, value } => test(deps, type_url, value),
        ExecuteMsg::Send { recipient, amount } => bank_send(env, recipient, amount),
    }
}

pub fn test(_deps: DepsMut, type_url: String, value: String) -> Result<Response<CustomMsg>, ContractError> {
    let msg = CosmosMsg::Stargate {
        type_url: type_url,
        value: to_binary(&value)?,
    };
    Ok(Response::new()
        .add_attribute("method", "test")
        .add_submessage(SubMsg::new(msg)))
}

pub fn bank_send(env: Env, recipient: String, amount: Uint128) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address.to_string();
    let msg: CosmosMsg = MsgSend {
        from_address: contract_addr.clone(),
        to_address: recipient,
        amount: vec![Coin {
            denom: "cony".to_string(),
            amount: amount.to_string(),
        },],
    }
    .into();

    // Ok(Response::new()
    //     .add_submessage(msg: SubMsg::new(msg)))
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "bank_send"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary("ok")
}
