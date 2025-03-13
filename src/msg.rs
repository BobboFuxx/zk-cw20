use cosmwasm_std::{Binary, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub token_name: String,
    pub token_symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Transfer {
        recipient_stealth_address: Addr,
        amount: u128,
        zk_proof: Binary,
    },
    GenerateStealthAddress {
        public_key: Binary,
    },
    SetViewingKey {
        key: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    Balance {
        address: Addr,
        viewing_key: String,
    },
    TransactionHistory {
        address: Addr,
        viewing_key: String,
    },
}
