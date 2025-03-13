use cosmwasm_std::{from_binary, to_binary, Addr, Coin, CosmosMsg, Empty, Uint128, StdError};
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
use privacy_token::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, TransferMsg};
use privacy_token::state::{TokenBalance, StealthAddress};

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to instantiate the contract with initial parameters
    fn contract_init() -> (App, Addr) {
        let mut app = App::default();
        let contract_code_id = app.store_code(ContractWrapper::new(contract::execute, contract::instantiate, contract::query));
        
        let init_msg = InstantiateMsg {
            token_name: "PrivacyToken".to_string(),
            token_symbol: "PTK".to_string(),
        };

        let contract_addr = app.instantiate_contract(
            contract_code_id,
            Addr::unchecked("creator"),
            &init_msg,
            &[],
            "Privacy Token",
            None,
        ).unwrap();

        (app, contract_addr)
    }

    #[test]
    fn test_initialization() {
        let (mut app, contract_addr) = contract_init();

        // Test contract initialization
        let result: InstantiateMsg = app.query_wasm_smart(contract_addr.clone(), &QueryMsg::Instantiate {}).unwrap();
        assert_eq!(result.token_name, "PrivacyToken");
        assert_eq!(result.token_symbol, "PTK");
    }

    #[test]
    fn test_transfer() {
        let (mut app, contract_addr) = contract_init();

        // Define user accounts
        let sender = Addr::unchecked("user1");
        let recipient = Addr::unchecked("user2");

        // Send tokens from user1 to user2
        let transfer_msg = ExecuteMsg::Transfer(TransferMsg {
            recipient_stealth_address: "stealth_address_2".to_string(),
            amount: Uint128::from(100u128),
            zk_proof: "zk_proof_data".to_string(),
        });

        // Execute transfer
        let res = app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &transfer_msg,
            &[],
        );
        
        // Verify successful transfer
        assert!(res.is_ok());
    }

    #[test]
    fn test_balance_query() {
        let (mut app, contract_addr) = contract_init();

        // Define user accounts
        let user_address = Addr::unchecked("user1");

        // Query the balance of user1
        let query_msg = QueryMsg::Balance {
            address: user_address.to_string(),
            viewing_key: "viewing_key_1".to_string(),
        };

        let res: TokenBalance = app.query_wasm_smart(contract_addr.clone(), &query_msg).unwrap();
        
        // Assert balance is returned and is initially 0 (since no tokens are minted yet)
        assert_eq!(res.balance, Uint128::zero());
    }

    #[test]
    fn test_stealth_address_generation() {
        let (mut app, contract_addr) = contract_init();

        // Define the public key for stealth address generation
        let public_key = "public_key_1";

        // Generate stealth address using the public key
        let query_msg = QueryMsg::StealthAddress {
            public_key: public_key.to_string(),
        };

        let res: StealthAddress = app.query_wasm_smart(contract_addr.clone(), &query_msg).unwrap();
        
        // Assert that the stealth address was generated correctly
        assert!(res.stealth_address.starts_with("stealth_"));
    }

    #[test]
    fn test_invalid_transfer() {
        let (mut app, contract_addr) = contract_init();

        // Define user accounts
        let sender = Addr::unchecked("user1");
        let recipient = Addr::unchecked("user2");

        // Attempt to transfer more tokens than available
        let transfer_msg = ExecuteMsg::Transfer(TransferMsg {
            recipient_stealth_address: "stealth_address_2".to_string(),
            amount: Uint128::from(1000u128), // assuming user has less than 1000 tokens
            zk_proof: "zk_proof_data".to_string(),
        });

        let res = app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &transfer_msg,
            &[],
        );
        
        // Assert that the transaction fails with an error due to insufficient funds
        match res {
            Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "Insufficient funds"),
            _ => panic!("Expected failure due to insufficient funds"),
        }
    }

    #[test]
    fn test_private_balance() {
        let (mut app, contract_addr) = contract_init();

        // Define user accounts
        let user_address = Addr::unchecked("user1");

        // Query the private balance of the user using the viewing key
        let query_msg = QueryMsg::Balance {
            address: user_address.to_string(),
            viewing_key: "viewing_key_1".to_string(),
        };

        let res: TokenBalance = app.query_wasm_smart(contract_addr.clone(), &query_msg).unwrap();
        
        // Assert the balance is returned correctly (even though private)
        assert!(res.balance > Uint128::zero());
    }
}
