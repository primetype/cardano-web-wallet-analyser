pub mod byron;
pub mod stake;
pub mod used_addresses;

use cardano_connector::{error::APIError, Address, ConnectedWallet, Utxo};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

pub enum AlertMessage {
    UTxOsWithByronAddress { utxos: HashMap<Address, Vec<Utxo>> },
    UTxOsWithUnknownRewardAddresses { utxos: HashMap<Address, Vec<Utxo>> },
    UTxOsWithUsedAddresses { utxos: HashMap<Address, Vec<Utxo>> },
    APIError { error: APIError },
}

pub async fn analyse_wallet(wallet: ConnectedWallet, alert: impl Fn(AlertMessage)) {
    let utxos = Arc::new(need!(wallet.all_utxos(None).await, alert));

    let () = byron::does_contain_byron_addresses(&utxos)
        .and_then(|utxos| Ok(alert(AlertMessage::UTxOsWithByronAddress { utxos })))
        .unwrap_or_else(|()| todo!());

    let change_address = need!(wallet.change_address().await, alert);

    let used_addresses = need!(wallet.used_addresses(None).await, alert);
    let used_addresses = used_addresses.into_iter().collect::<HashSet<_>>();
    let used_addresses = Arc::new(used_addresses);

    let unused_addresses = need!(wallet.unused_addresses().await, alert);
    let unused_addresses = unused_addresses.into_iter().collect::<HashSet<_>>();
    let unused_addresses = Arc::new(unused_addresses);

    let reward_addresses = need!(wallet.reward_addresses().await, alert);
    let reward_addresses = reward_addresses.into_iter().collect::<HashSet<_>>();
    let reward_addresses = Arc::new(reward_addresses);

    let () = stake::consistent_reward_keys(&reward_addresses, &utxos)
        .and_then(|utxos| {
            Ok(alert(AlertMessage::UTxOsWithUnknownRewardAddresses {
                utxos,
            }))
        })
        .unwrap_or_else(|()| todo!());

    let () = used_addresses::no_used_addresses(&used_addresses, &utxos)
        .and_then(|utxos| Ok(alert(AlertMessage::UTxOsWithUsedAddresses { utxos })))
        .unwrap_or_else(|()| todo!());
}

macro_rules! need {
    ($QUERY:expr, $ALERT:ident) => {{
        match $QUERY {
            Ok(res) => res,
            Err(error) => {
                $ALERT(AlertMessage::APIError { error });
                return;
            }
        }
    }};
}

macro_rules! decode_address {
    ($ADDRESS:expr) => {{
        match $ADDRESS {
            Ok(address) => address,
            Err(error) => {
                ::gloo::console::error!(error.to_string());

                continue;
            }
        }
    }};
}

pub(self) use decode_address;
pub(self) use need;
