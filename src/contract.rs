#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, from_binary };
use cw2::{ set_contract_version, get_contract_version };
use cw721::{ Cw721ReceiveMsg };

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, NftReceiveMsg, MigrateMsg };
use crate::state::{ Config, CONFIG, StakingInfo, STAKING, UNLOCK_TIME };
use crate::util::{ check_enabled, check_owner, get_transfer_message };

// version info for migration info
const CONTRACT_NAME: &str = "SEITIZEN_STAKING";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = Config {
        collection_address: msg.collection_address.clone(),
        owner: info.sender.clone(),
        lock_time: msg.lock_time,
        enabled: false,
        total_supply: 0u64
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("collection_address", msg.collection_address.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwner { owner } => execute_update_owner(deps, info, owner),
        ExecuteMsg::UpdateCollectionAddress { collection_address } => execute_update_collection(deps, info, collection_address),
        ExecuteMsg::UpdateLockTime { lock_time } => execute_update_locktime(deps, info, lock_time),
        ExecuteMsg::UpdatedEnabled { enabled } => execute_update_enabled(deps, info, enabled),
        ExecuteMsg::ReceiveNft(msg) => execute_receive_nft(deps, info, msg),
        ExecuteMsg::Unstake { token_id } => execute_unstake(deps, env, info, token_id),
        ExecuteMsg::WithdrawId { token_id } => execute_withdraw(deps, info, token_id)
    }
}

pub fn execute_update_owner(
    deps: DepsMut,
    info: MessageInfo,
    owner: Addr,
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender)?;
    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.owner = owner.clone();
        Ok(exists)
    })?;
    Ok(Response::new().add_attribute("action", "update_owner").add_attribute("owner", owner))
}

pub fn execute_update_enabled(
    deps: DepsMut,
    info: MessageInfo,
    enabled: bool
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender)?;
    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.enabled = enabled;
        Ok(exists)
    })?;
    Ok(Response::new().add_attribute("action", "update_enabled").add_attribute("enabled", enabled.to_string()))
}

pub fn execute_update_collection(
    deps: DepsMut,
    info: MessageInfo,
    collection_address: Addr,
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender)?;
    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.collection_address = collection_address.clone();
        Ok(exists)
    })?;
    Ok(Response::new().add_attribute("action", "update_collection").add_attribute("collection_address", collection_address))
}

pub fn execute_update_locktime(
    deps: DepsMut,
    info: MessageInfo,
    lock_time: u64
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender)?;
    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.lock_time = lock_time;
        Ok(exists)
    })?;
    Ok(Response::new().add_attribute("action", "update_lockTime").add_attribute("lockTime", lock_time.to_string()))
}

pub fn execute_receive_nft(
    deps: DepsMut,
    info: MessageInfo,
    msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    check_enabled(deps.storage)?;
    let cfg = CONFIG.load(deps.storage)?;

    if info.sender.clone() != cfg.collection_address {
        return Err(ContractError::InvalidCw721 {});
    }
    let token_id = msg.token_id.clone();
    let user_addr = deps.api.addr_validate(msg.sender.as_str())?;

    let nft_msg: NftReceiveMsg = from_binary(&msg.msg)?;

    match nft_msg {
        NftReceiveMsg::Stake {} => {
            let mut record = StakingInfo {
                address: user_addr.clone(),
                token_ids: vec![token_id.clone()],
            };
            if STAKING.has(deps.storage, user_addr.clone()) {
                record = STAKING.load(deps.storage, user_addr.clone())?;
                let mut list = record.token_ids.clone();
                list.push(token_id.clone());
                record.token_ids = list;
            }
            STAKING.save(deps.storage, user_addr.clone(), &record)?;
            CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
                exists.total_supply = cfg.total_supply + 1;
                Ok(exists)
            })?;
            Ok(Response::new()
                .add_attribute("action", "execute_receive")
                .add_attribute("token_id", token_id)
                .add_attribute("user_address", user_addr)
            )
        },
        _ => {
            return Err(ContractError::InvalidStaking {})
        },
    }
}

pub fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String
) -> Result<Response, ContractError> {
    let mut cfg = CONFIG.load(deps.storage)?;
    if cfg.lock_time == 0u64 {
        let stake_info = STAKING.may_load(deps.storage, info.sender.clone())?;
        if stake_info.is_none() {
            return Err(ContractError::InvalidStaking {});
        }
        let mut stake_info = stake_info.unwrap();
        if stake_info.token_ids.contains(&token_id) {
            let mut token_ids_vec = stake_info.token_ids;
            token_ids_vec.retain(|x| x != &token_id);
            stake_info.token_ids = token_ids_vec;
            STAKING.save(deps.storage, info.sender.clone(), &stake_info)?;
            let send_nft_msg = get_transfer_message(cfg.clone().collection_address, token_id.clone(), info.sender)?;
            CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
                exists.total_supply = cfg.total_supply - 1;
                Ok(exists)
            })?;
            return Ok(Response::new()
                .add_message(send_nft_msg)
                .add_attribute("action", "unstake")
                .add_attribute("token_id", token_id.clone())
            );
        } else {
            return Err(ContractError::InvalidStaking {});
        }
    } else {
        let unlocked_time = UNLOCK_TIME.may_load(deps.storage, token_id.clone())?;
        if unlocked_time.is_none() {
            UNLOCK_TIME.save(deps.storage, token_id.clone(), &env.block.time.seconds())?;
            return Ok(Response::new()
                .add_attribute("action", "create_unstake")
                .add_attribute("token_id", token_id.clone())
            );
        } else {
            let unlocked_time = unlocked_time.unwrap();
            if env.block.time.seconds() > unlocked_time + cfg.lock_time {
                let stake_info = STAKING.may_load(deps.storage, info.sender.clone())?;
                if stake_info.is_none() {
                    return Err(ContractError::InvalidStaking {});
                }
                let mut stake_info = stake_info.unwrap();
                if stake_info.token_ids.contains(&token_id) {
                    let mut token_ids_vec = stake_info.token_ids;
                    token_ids_vec.retain(|x| x != &token_id);
                    stake_info.token_ids = token_ids_vec;
                    STAKING.save(deps.storage, info.sender.clone(), &stake_info)?;
                    let send_nft_msg = get_transfer_message(cfg.clone().collection_address, token_id.clone(), info.sender)?;
                    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
                        exists.total_supply = cfg.total_supply - 1;
                        Ok(exists)
                    })?;
                    return Ok(Response::new()
                        .add_message(send_nft_msg)
                        .add_attribute("action", "unstake")
                        .add_attribute("token_id", token_id.clone())
                    );
                } else {
                    return Err(ContractError::InvalidStaking {});
                }
            } else {
                return Err(ContractError::PendingUnStaking {});
            }
        }
    }
}

pub fn execute_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender.clone())?;
    let config = CONFIG.load(deps.storage)?;

    let cw721_address = config.clone().collection_address;
    let msg = get_transfer_message(cw721_address, token_id.clone(), info.sender)?;
    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.total_supply = config.total_supply - 1;
        Ok(exists)
    })?;
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "withdraw")
        .add_attribute("token_id", token_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetStaking { address } => to_binary(&query_staking_info(deps, address)?)
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn query_staking_info(deps: Deps, address: Addr) -> StdResult<StakingInfo> {
    let staking_info = STAKING.load(deps.storage, address)?;
    Ok(staking_info)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, crate::ContractError> {
    let version = get_contract_version(deps.storage)?;
    if version.contract != CONTRACT_NAME {
        return Err(ContractError::CannotMigrate {
            previous_contract: version.contract,
        });
    }
    Ok(Response::default())
}

#[test]
fn test() {
    let mut string_array: Vec<String> = Vec::new();
    string_array.push ("result1".to_string());
    string_array.push ("result2".to_string());
    string_array.push ("result3".to_string());

    string_array.retain_mut(|x| x!=&String::from("result1"));
    println!("{:?}", string_array)
}