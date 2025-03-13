use secp256k1::{PublicKey, SecretKey, Secp256k1};
use cosmwasm_std::{DepsMut, StdResult, Binary};
use rand::rngs::OsRng;

pub fn generate_stealth_address(pub_key: Binary) -> StdResult<Binary> {
    let secp = Secp256k1::new();
    let recipient_pubkey = PublicKey::from_slice(&pub_key.0).unwrap();
    let random_secret = SecretKey::new(&mut OsRng);
    let stealth_address = recipient_pubkey.mul_tweak(&secp, &random_secret).unwrap();
    Ok(Binary(stealth_address.serialize().to_vec()))
}
