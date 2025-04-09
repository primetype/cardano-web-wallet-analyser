//! connect to a cardano wallet extension in the browser
//!

mod body;
mod context;
pub mod icons;
mod wallet;

pub use self::{
    body::AppBody,
    context::{AppContextProvider, ContextProviderProps},
    icons::*,
};
use context::Action;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct WalletButtonProps {
    pub wallet: cardano_connector::Wallet,
}

#[function_component]
fn WalletButton(props: &WalletButtonProps) -> Html {
    let ctx = use_context::<context::ContextHandle>().unwrap();

    let wallet_app_name = props.wallet.name();
    let wallet_app_icon = props.wallet.icon();

    let selected = ctx
        .wallet
        .as_ref()
        .map(|wallet| wallet.name() == wallet_app_name)
        .unwrap_or_default();

    let onclick = {
        let ctx = ctx.clone();
        let wallet = props.wallet.clone();

        move |_| ctx.dispatch(Action::WalletSelected(wallet.clone()))
    };

    let active = if selected { Some("active") } else { None };

    html! {
        <button
            class={classes!("btn", "btn-light", "d-flex", "flex-fill", "mb-3", active)}
            onclick={onclick}>
            <img src={wallet_app_icon} alt={format!("{} logo", wallet_app_name)} class="me-3" />
            <h4 class="mb-0">{&wallet_app_name}</h4>
        </button>
    }
}

/// display a list of available wallets
///
/// The user can select a wallet to connect to the application.
#[function_component(WalletList)]
pub fn wallet_list() -> Html {
    let wallets = cardano_connector::wallets();

    if wallets.is_empty() {
        html! {
            <div class="col d-flex">
                <div class="alert alert-warning d-flex align-items-center" role="alert">
                    <h4 class="alert-heading">{"No CIP30 Wallets Found"}</h4>
                    <p>{"We couldn't find any CIP30-compatible wallets installed in your browser. "}
                    {"To interact with this application, please install "}
                    <a href="https://www.lace.io" class="alert-link">{"Lace wallet"}</a>
                    {" and try again."}</p>
                </div>
            </div>
        }
    } else {
        html! {
            <>
                {
                    for wallets.iter().map(|wallet|
                        html!{
                            <div class="col d-flex">
                                <WalletButton wallet={wallet.clone()} />
                            </div>
                        }
                    )
                }
            </>
        }
    }
}
