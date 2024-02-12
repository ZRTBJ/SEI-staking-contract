use cosmwasm_std::{ Response, Uint128, Storage, Addr, CosmosMsg, WasmMsg, to_binary };
use cw721::{ Cw721ExecuteMsg };
use crate::error::ContractError;
use crate::state::CONFIG;


pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_ORDER: u64 = 10;

pub fn multiple() -> Uint128 { Uint128::from(100u128) }
pub fn decimal() -> Uint128 { Uint128::from(1000000u128) }

pub fn check_enabled(
    storage: &mut dyn Storage,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(storage)?;
    if !cfg.enabled {
        return Err(ContractError::Disabled {})
    }
    Ok(Response::new())
}

pub fn check_owner(
    storage: &mut dyn Storage,
    address: Addr
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(storage)?;
    
    if address != cfg.owner {
        return Err(ContractError::Unauthorized {})
    }
    Ok(Response::new())
}

pub fn get_transfer_message(
    collection_address: Addr,
    token_id: String,
    receiver: Addr
) -> Result<CosmosMsg, ContractError> {
    return Ok(CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr: collection_address.to_string(), 
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            token_id,
            recipient: receiver.into(),
        })?, 
        funds: vec![] }))
}