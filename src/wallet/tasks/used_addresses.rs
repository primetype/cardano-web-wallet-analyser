use std::collections::{HashMap, HashSet};

use super::decode_address;
use cardano_connector::{Address, Utxo};

pub fn no_used_addresses(
    used_addresses: &HashSet<Address>,
    utxos: &[Utxo],
) -> Result<HashMap<Address, Vec<Utxo>>, ()> {
    let mut found = HashMap::new();

    if used_addresses.is_empty() {
        return Ok(found);
    }

    for utxo in utxos {
        let address = decode_address!(utxo.address());

        match address {
            // byron addresses don't have any stake keys
            Address::Byron(..) => continue,

            Address::Shelley(address) => {
                let stake_address = match address.try_into() {
                    Ok(addr) => Address::Stake(addr),
                    Err(error) => {
                        // TODO;
                        gloo::console::error!(error.to_string());
                        continue;
                    }
                };

                if !used_addresses.contains(&stake_address) {
                    found.entry(stake_address).or_default().push(utxo.clone());
                }
            }

            stake_address @ Address::Stake(..) => {
                if !used_addresses.contains(&stake_address) {
                    found.entry(stake_address).or_default().push(utxo.clone());
                }
            }
        }
    }

    Ok(found)
}
