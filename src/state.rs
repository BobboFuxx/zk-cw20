use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

const BALANCES: &[u8] = b"balances";
const VIEWING_KEYS: &[u8] = b"viewing_keys";

pub fn set_balance(storage: &mut dyn Storage, addr: &Addr, balance: u128) {
    let mut balances = PrefixedStorage::new(BALANCES, storage);
    balances.set(addr.as_bytes(), &balance.to_le_bytes());
}

pub fn get_balance(storage: &dyn Storage, addr: &Addr, viewing_key: &str) -> Option<u128> {
    let balances = ReadonlyPrefixedStorage::new(BALANCES, storage);
    let keys = ReadonlyPrefixedStorage::new(VIEWING_KEYS, storage);

    if let Some(stored_key) = keys.get(addr.as_bytes()) {
        if stored_key == viewing_key.as_bytes() {
            if let Some(balance_bytes) = balances.get(addr.as_bytes()) {
                return Some(u128::from_le_bytes(balance_bytes.try_into().unwrap()));
            }
        }
    }
    None
}
