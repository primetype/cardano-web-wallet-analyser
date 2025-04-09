mod balance;
mod network_id;
mod tasks;
mod utxos;
mod views;

use std::collections::HashMap;

use self::{
    network_id::WalletNetworkId,
    tasks::{analyse_wallet, AlertMessage},
    utxos::UtxosView,
    views::{ByronAddressView, InconsistentRewardAddressesView, UsedAddressesView},
};
use cardano_connector::{Address, ConnectedWallet, Utxo};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Default)]
pub struct Wallet {
    state: State,

    utxos_with_byron_addresses: Option<HashMap<Address, Vec<Utxo>>>,
    utxos_with_unknown_reward_addresses: Option<HashMap<Address, Vec<Utxo>>>,
    utxos_with_used_addresses: Option<HashMap<Address, Vec<Utxo>>>,
}

#[derive(Default)]
enum State {
    #[default]
    NotConnected,
    Connecting,
    Connected(ConnectedWallet),
    Error(String),
}

pub enum WalletMessage {
    Connect,
    Connected(ConnectedWallet),
    ConnectionFailed(String),
    Alert(AlertMessage),
}

#[derive(Properties, PartialEq)]
pub struct WalletProperties {
    pub wallet: cardano_connector::Wallet,
}

impl Component for Wallet {
    type Message = WalletMessage;
    type Properties = WalletProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let wallet = ctx.props().wallet.clone();
        let link = ctx.link().clone();

        // check that the wallet is already connected with our app
        // before asking to go any further. This is because we do
        // not want to ask the user to connect to our dapp already
        // otherwise that would be a poor UX as we would require the
        // user to do something with the wallet extension before we
        // even finished loading the page.
        spawn_local(async move {
            if wallet.enabled().await.unwrap_or(false) {
                link.send_message(WalletMessage::Connect);
            }
        });

        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            WalletMessage::Connect => {
                let wallet = ctx.props().wallet.clone();

                spawn_local(async move {
                    match wallet.enable().await {
                        Ok(api) => link.send_message(WalletMessage::Connected(api)),
                        Err(error) => {
                            link.send_message(WalletMessage::ConnectionFailed(error.to_string()))
                        }
                    }
                });

                let old_state = std::mem::replace(&mut self.state, State::Connecting);
                *self = Self {
                    state: State::Connecting,
                    ..Default::default()
                };

                !matches!(old_state, State::Connecting)
            }
            WalletMessage::Connected(cip30_api) => {
                spawn_local(analyse_wallet(cip30_api.clone(), move |message| {
                    link.send_message(WalletMessage::Alert(message));
                }));

                *self = Self {
                    state: State::Connected(cip30_api),
                    ..Default::default()
                };
                true
            }
            WalletMessage::ConnectionFailed(error) => {
                *self = Self {
                    state: State::Error(error),
                    ..Default::default()
                };
                true
            }
            WalletMessage::Alert(AlertMessage::APIError { error }) => {
                self.state = State::Error(error.to_string());
                true
            }
            WalletMessage::Alert(AlertMessage::UTxOsWithByronAddress { utxos }) => {
                self.utxos_with_byron_addresses = Some(utxos);
                true
            }
            WalletMessage::Alert(AlertMessage::UTxOsWithUnknownRewardAddresses { utxos }) => {
                self.utxos_with_unknown_reward_addresses = Some(utxos);
                true
            }
            WalletMessage::Alert(AlertMessage::UTxOsWithUsedAddresses { utxos }) => {
                self.utxos_with_used_addresses = Some(utxos);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let wallet_app_icon = ctx.props().wallet.icon().clone();
        let wallet_app_name = ctx.props().wallet.name().clone();
        let wallet_app_version = ctx.props().wallet.version().clone();

        let body_content = match &self.state {
            State::NotConnected => {
                html! {
                    <button onclick={move |_| link.send_message(WalletMessage::Connect)} class="btn btn-primary">{"Connect Wallet"}</button>
                }
            }
            State::Connecting => {
                html! {
                    <div class="d-flex align-items-center">
                        <div class="spinner-border spinner-border-sm me-2" role="status">
                            <span class="visually-hidden">{"Loading..."}</span>
                        </div>
                        <span>{"Connecting..."}</span>
                    </div>
                }
            }
            State::Connected(wallet) => {
                html! {
                    <>
                        <UtxosView wallet={wallet.clone()} />

                        <div class="d-flex justify-content-between align-items-center my-4">
                            <h4>{"Wallet Overview"}</h4>
                        </div>

                        <ByronAddressView utxos={self.utxos_with_byron_addresses.clone()} />
                        <InconsistentRewardAddressesView utxos={self.utxos_with_unknown_reward_addresses.clone()} />
                        <UsedAddressesView utxos={self.utxos_with_used_addresses.clone()} />
                    </>
                }
            }
            State::Error(error) => {
                html! {
                    <>
                        <div class="alert alert-danger" role="alert">
                            {format!("Connection error: {}", error)}
                        </div>
                        <button onclick={move |_| link.send_message(WalletMessage::Connect)} class="btn btn-primary">{"Try Again"}</button>
                    </>
                }
            }
        };

        let network_id = if let State::Connected(wallet) = &self.state {
            html! { <WalletNetworkId wallet={wallet.clone()} /> }
        } else {
            html! {}
        };

        html! {
            <div class="card">
                <div class="card-header">
                    <div class="d-flex align-items-center">
                        <img src={wallet_app_icon} alt="wallet icon" width="32" height="32" class="me-2"/>
                        <h3 class="card-title mb-0 me-2">
                            {wallet_app_name}
                        </h3>
                        <span class="badge rounded-pill text-bg-secondary">{wallet_app_version}</span>
                        {network_id}
                    </div>
                </div>
                <div class="card-body">
                    {body_content}
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let new_wallet_app_icon = ctx.props().wallet.icon().clone();
        let old_wallet_app_icon = old_props.wallet.icon().clone();

        if new_wallet_app_icon != old_wallet_app_icon {
            *self = Self::default();

            let wallet = ctx.props().wallet.clone();
            let link = ctx.link().clone();
            spawn_local(async move {
                if wallet.enabled().await.unwrap_or(false) {
                    link.send_message(WalletMessage::Connect);
                }
            });
            true
        } else {
            false
        }
    }
}
