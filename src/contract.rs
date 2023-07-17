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
    let b = value.as_bytes();
    let mut r = String::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 1 < b.len() && b[i+1] == b'x' {
            if i + 3 < b.len() {
                let hex_digits = &b[(i+2)..(i+4)];
                if let Ok(ascii_byte) = u8::from_str_radix(std::str::from_utf8(hex_digits).unwrap(), 16) {
                    r.push(ascii_byte as char);
                    i += 3;
                } else {
                    r.push_str("\\x");
                    i += 1;
                }
            } else {
                r.push_str("\\x");
                i += 1;
            }
        } else {
            r.push(b[i] as char);
        }
        i += 1;
    }

    let msg = CosmosMsg::Stargate { 
        type_url: type_url, 
        value: Binary::from(r.as_bytes()),
    };
    Ok(Response::new()
        .add_attribute("method", "test")
        .add_submessage(SubMsg::new(msg)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary("ok")
}
