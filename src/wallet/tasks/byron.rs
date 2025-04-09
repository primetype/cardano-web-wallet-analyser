use super::decode_address;
use cardano_connector::{Address, Utxo};
use std::collections::HashMap;

pub fn does_contain_byron_addresses(utxos: &[Utxo]) -> Result<HashMap<Address, Vec<Utxo>>, ()> {
    let mut found = HashMap::new();

    for utxo in utxos {
        let address = decode_address!(utxo.address());

        if matches!(address, Address::Byron(..)) {
            found
                .entry(address)
                .or_insert_with(Vec::new)
                .push(utxo.clone());
        }
    }

    Ok(found)
}
