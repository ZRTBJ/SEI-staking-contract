// use cw_multi_test::{ App, Contract, ContractWrapper, Executor };
// use cosmwasm_std::{ Empty, testing::mock_env, Addr, StdResult, Timestamp, BlockInfo, to_binary };
// use cw721_base::{msg::{ InstantiateMsg as Cw721InstantiateMsg, ExecuteMsg as Cw721ExecuteMsg, QueryMsg as Cw721QueryMsg }, MintMsg};
// use cw721::{ OwnerOfResponse, AllNftInfoResponse, NumTokensResponse };
// use crate::{msg::{ InstantiateMsg, QueryMsg, NftReceiveMsg, ExecuteMsg }, state::StakingInfo};
// use crate::state::Config;

// pub type Extension = Option<Empty>;

// fn mock_app() -> App {
//     App::default()
// }
// pub fn cw721_contract() -> Box<dyn Contract<Empty>> {
//     let contract = ContractWrapper::new(
//         cw721_base::entry::execute, 
//         cw721_base::entry::instantiate, 
//         cw721_base::entry::query);
//     Box::new(contract)
// }

// pub fn staking_contract() -> Box<dyn Contract<Empty>> {
//     let contract = ContractWrapper::new(
//         crate::contract::execute,
//         crate::contract::instantiate,
//         crate::contract::query
//     );
//     Box::new(contract)
// }

// fn init_cw721_contract(router: &mut App) -> Addr {
//     let msg = Cw721InstantiateMsg {
//         name: "SEITIZEN".to_string(),
//         symbol: "SEITIZEN".to_string(),
//         minter: Addr::unchecked("admin").to_string()
//     };
//     let cw721_id = router.store_code(cw721_contract());

//     let cw721_address = router
//         .instantiate_contract(
//             cw721_id, 
//             Addr::unchecked("admin"),
//             &msg, 
//             &[], 
//             "CW721", 
//             Some("admin".to_string())
//         ).unwrap();
//     cw721_address
// }

// fn init_staking_contract(router: &mut App, collection: Addr, lock_time: u64) -> Addr {
//     let msg = InstantiateMsg {
//         lock_time: lock_time,
//         collection_address: collection
//     };
//     let staking_id = router.store_code(staking_contract());
//     let staking_address = router
//         .instantiate_contract(
//             staking_id, 
//             Addr::unchecked("admin"),
//             &msg, 
//             &[], 
//             "STAKING", 
//             Some("admin".to_string())
//         ).unwrap();
//     staking_address
// }

// fn mint_nft(router: &mut App, cw721_address: Addr, receiver: Addr, token_id: String) -> StdResult<()> {
//     router.execute_contract(
//         receiver.clone(), 
//         cw721_address.clone(), 
//         &Cw721ExecuteMsg::Mint(MintMsg::<Extension>{ 
//             token_id: token_id, 
//             owner: receiver.to_string(), 
//             token_uri: None, 
//             extension: None
//         }), 
//         &[]
//     ).unwrap();
//     Ok(())
// }
// fn stake_nft(router: &mut App, cw721_address: Addr, from_address: Addr, to_address: Addr, token_id: String) -> StdResult<()> {
//     router.execute_contract(
//         from_address, 
//         cw721_address, 
//         &Cw721ExecuteMsg::<Extension>::SendNft { 
//             contract: to_address.to_string(), 
//             token_id: token_id, 
//             msg: to_binary(&NftReceiveMsg::Stake {})?
//         }, 
//         &[]
//     ).unwrap();    
//     Ok(())
// }

// fn get_nft_owner(router: &mut App, cw721_address: &Addr, token_id: &String) -> String {
//     let nft_info: OwnerOfResponse = router
//         .wrap()
//         .query_wasm_smart(
//             cw721_address, 
//             &Cw721QueryMsg::OwnerOf { token_id: token_id.to_string(), include_expired: None }
//         )
//         .unwrap();
//     nft_info.owner
// }


// #[test]
// fn test_staking_without_locking_time() {
//     let mut router = mock_app();
//     let mut env = mock_env();
//     let admin = Addr::unchecked("admin");
//     let user = Addr::unchecked("user");
//     let cw721_token = init_cw721_contract(&mut router);
//     let cw721_token_2 = init_cw721_contract(&mut router);
//     let staking_contract = init_staking_contract(&mut router, cw721_token.clone(), 0u64);
//     let ts = Timestamp::from_seconds(1);
//     router.set_block(BlockInfo {
//         height: 1,
//         time: ts,
//         chain_id: "1".to_string()
//     });
//     mint_nft(&mut router, cw721_token.clone(), admin.clone(), "1".to_string()).unwrap();
//     mint_nft(&mut router, cw721_token.clone(), admin.clone(), "2".to_string()).unwrap();
//     mint_nft(&mut router, cw721_token_2.clone(), admin.clone(), "1".to_string()).unwrap();
//     // let owner = get_nft_info(&mut router, &cw721_token, &"1".to_string());
//     // let staking_config: Config = router
//     //     .wrap()
//     //     .query_wasm_smart(
//     //         staking_contract.clone(), 
//     //         &QueryMsg::GetConfig {}
//     //     ).unwrap();
    
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::UpdatedEnabled { enabled: true }, 
//         &[]
//     ).unwrap();
    
    
//     let _ = stake_nft(&mut router, cw721_token.clone(), admin.clone(), staking_contract.clone(), "1".to_string());
//     let _ = stake_nft(&mut router, cw721_token.clone(), admin.clone(), staking_contract.clone(), "2".to_string());
//     // let _ = stake_nft(&mut router, cw721_token_2.clone(), admin.clone(), staking_contract.clone(), "1".to_string());  // should be reverted with invalidNFT.

//     // let stake_info: StakingInfo = router
//     //     .wrap()
//     //     .query_wasm_smart(
//     //         staking_contract.clone(), 
//     //         &QueryMsg::GetStaking { address: admin.clone() }
//     //     ).unwrap();

//     let nft_owner = get_nft_owner(&mut router, &cw721_token.clone(), &"1".to_string());

//     println!("---------------------------nft-owner{:?}", nft_owner);
//     let staking_config: Config = router
//         .wrap()
//         .query_wasm_smart(
//             staking_contract.clone(), 
//             &QueryMsg::GetConfig {}
//         ).unwrap();
//     println!("--------------------total-supply: {:?}", staking_config.total_supply);   
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::Unstake { token_id: "1".to_string() }, 
//         &[]
//     ).unwrap();
//     let staking_config: Config = router
//         .wrap()
//         .query_wasm_smart(
//             staking_contract.clone(), 
//             &QueryMsg::GetConfig {}
//         ).unwrap();
//     println!("--------------------total-supply: {:?}", staking_config.total_supply);
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::Unstake { token_id: "2".to_string() }, 
//         &[]
//     ).unwrap();

//     let stake_info: StakingInfo = router
//         .wrap()
//         .query_wasm_smart(
//             staking_contract.clone(), 
//             &QueryMsg::GetStaking { address: admin.clone() }
//         ).unwrap();
//     let staking_config: Config = router
//         .wrap()
//         .query_wasm_smart(
//             staking_contract.clone(), 
//             &QueryMsg::GetConfig {}
//         ).unwrap();
//     println!("--------------------total-supply: {:?}", staking_config.total_supply);
//     let nft_owner = get_nft_owner(&mut router, &cw721_token.clone(), &"1".to_string());

//     println!("---------------------------nft-owner{:?}", nft_owner);
//     println!("---------------------------{:?}", stake_info.token_ids);
    
// }

// #[test]
// fn test_staking_with_locking_time() {
//     let mut router = mock_app();
//     let mut env = mock_env();
//     let admin = Addr::unchecked("admin");
//     let user = Addr::unchecked("user");
//     let cw721_token = init_cw721_contract(&mut router);
//     let cw721_token_2 = init_cw721_contract(&mut router);
//     let staking_contract = init_staking_contract(&mut router, cw721_token.clone(), 100u64);
//     let ts = Timestamp::from_seconds(1);
//     router.set_block(BlockInfo {
//         height: 1,
//         time: ts,
//         chain_id: "1".to_string()
//     });
//     mint_nft(&mut router, cw721_token.clone(), admin.clone(), "1".to_string()).unwrap();
//     mint_nft(&mut router, cw721_token.clone(), admin.clone(), "2".to_string()).unwrap();
//     mint_nft(&mut router, cw721_token_2.clone(), admin.clone(), "1".to_string()).unwrap();
//     // let owner = get_nft_info(&mut router, &cw721_token, &"1".to_string());
//     // let staking_config: Config = router
//     //     .wrap()
//     //     .query_wasm_smart(
//     //         staking_contract.clone(), 
//     //         &QueryMsg::GetConfig {}
//     //     ).unwrap();
    
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::UpdatedEnabled { enabled: true }, 
//         &[]
//     ).unwrap();
    
    
//     let _ = stake_nft(&mut router, cw721_token.clone(), admin.clone(), staking_contract.clone(), "1".to_string());
//     let _ = stake_nft(&mut router, cw721_token.clone(), admin.clone(), staking_contract.clone(), "2".to_string());
//     // let _ = stake_nft(&mut router, cw721_token_2.clone(), admin.clone(), staking_contract.clone(), "1".to_string());   // should be reverted with invalidNFT.

//     // let stake_info: StakingInfo = router
//     //     .wrap()
//     //     .query_wasm_smart(
//     //         staking_contract.clone(), 
//     //         &QueryMsg::GetStaking { address: admin.clone() }
//     //     ).unwrap();

//     let nft_owner = get_nft_owner(&mut router, &cw721_token.clone(), &"1".to_string());

//     println!("---------------------------nft-owner{:?}", nft_owner);
    
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::Unstake { token_id: "1".to_string() }, 
//         &[]
//     ).unwrap();

//     let stake_info: StakingInfo = router
//         .wrap()
//         .query_wasm_smart(
//             staking_contract.clone(), 
//             &QueryMsg::GetStaking { address: admin.clone() }
//         ).unwrap();
    
//     let nft_owner = get_nft_owner(&mut router, &cw721_token.clone(), &"1".to_string());

//     println!("---------------------------nft-owner{:?}", nft_owner);
//     println!("---------------------------{:?}", stake_info.token_ids);
//     router.set_block(BlockInfo {
//         height: 10,
//         time: Timestamp::from_seconds(200),
//         chain_id: "1".to_string()
//     });
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::Unstake { token_id: "1".to_string() }, 
//         &[]
//     ).unwrap();    

//     let stake_info: StakingInfo = router
//     .wrap()
//     .query_wasm_smart(
//         staking_contract.clone(), 
//         &QueryMsg::GetStaking { address: admin.clone() }
//     ).unwrap();

//     let nft_owner = get_nft_owner(&mut router, &cw721_token.clone(), &"1".to_string());

//     println!("---------------------------nft-owner{:?}", nft_owner);
//     println!("---------------------------{:?}", stake_info.token_ids);
// }


// #[test]
// fn test_withdraw_nft() {
//     let mut router = mock_app();
//     let mut env = mock_env();
//     let admin = Addr::unchecked("admin");
//     let user = Addr::unchecked("user");
//     let cw721_token = init_cw721_contract(&mut router);
//     let cw721_token_2 = init_cw721_contract(&mut router);
//     let staking_contract = init_staking_contract(&mut router, cw721_token.clone(), 100u64);
//     let ts = Timestamp::from_seconds(1);
//     router.set_block(BlockInfo {
//         height: 1,
//         time: ts,
//         chain_id: "1".to_string()
//     });
//     mint_nft(&mut router, cw721_token.clone(), user.clone(), "1".to_string()).unwrap();
//     mint_nft(&mut router, cw721_token.clone(), user.clone(), "2".to_string()).unwrap();
//     mint_nft(&mut router, cw721_token_2.clone(), user.clone(), "1".to_string()).unwrap();
//     // let owner = get_nft_info(&mut router, &cw721_token, &"1".to_string());
//     // let staking_config: Config = router
//     //     .wrap()
//     //     .query_wasm_smart(
//     //         staking_contract.clone(), 
//     //         &QueryMsg::GetConfig {}
//     //     ).unwrap();
    
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::UpdatedEnabled { enabled: true }, 
//         &[]
//     ).unwrap();
    
    
//     let _ = stake_nft(&mut router, cw721_token.clone(), user.clone(), staking_contract.clone(), "1".to_string());
//     let _ = stake_nft(&mut router, cw721_token.clone(), user.clone(), staking_contract.clone(), "2".to_string());
//     // let _ = stake_nft(&mut router, cw721_token_2.clone(), admin.clone(), staking_contract.clone(), "1".to_string());   // should be reverted with invalidNFT.

//     // let stake_info: StakingInfo = router
//     //     .wrap()
//     //     .query_wasm_smart(
//     //         staking_contract.clone(), 
//     //         &QueryMsg::GetStaking { address: admin.clone() }
//     //     ).unwrap();

//     let staking_config: Config = router
//         .wrap()
//         .query_wasm_smart(
//             staking_contract.clone(), 
//             &QueryMsg::GetConfig {}
//         ).unwrap();
//     println!("--------------------total-supply: {:?}", staking_config.total_supply);

//     let nft_owner = get_nft_owner(&mut router, &cw721_token.clone(), &"1".to_string());

//     println!("---------------------------nft-owner{:?}", nft_owner);
    
//     router.execute_contract(
//         admin.clone(), 
//         staking_contract.clone(), 
//         &ExecuteMsg::WithdrawId { token_id: "1".to_string() }, 
//         &[]
//     ).unwrap();
//     let nft_owner = get_nft_owner(&mut router, &cw721_token.clone(), &"1".to_string());

//     let staking_config: Config = router
//     .wrap()
//     .query_wasm_smart(
//         staking_contract.clone(), 
//         &QueryMsg::GetConfig {}
//     ).unwrap();
//     println!("--------------------total-supply: {:?}", staking_config.total_supply);

//     println!("---------------------------nft-owner{:?}", nft_owner);
// }