# zk-cw20
Shielded cosmWASM token

privacy_token/
│── src/
│   ├── contract.rs        # Main contract logic
│   ├── msg.rs             # Message types (InstantiateMsg, ExecuteMsg, QueryMsg)
│   ├── state.rs           # Storage (Balances, Viewing Keys, etc.)
│   ├── stealth.rs         # Stealth address generation logic
│   ├── zk_snark.rs        # zk-SNARK proof verification
│   ├── lib.rs             # Boilerplate CosmWasm setup
│── Cargo.toml             # Dependencies
│── schema/                # JSON schema for the contract (auto-generated)
│── tests/                 # Unit and integration tests


