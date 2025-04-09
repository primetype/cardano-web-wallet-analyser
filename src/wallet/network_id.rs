use cardano_connector::{ConnectedWallet, NetworkId};
use yew::{platform::spawn_local, prelude::*};

#[derive(Default)]
pub struct WalletNetworkId {
    state: NetworkIdState,
}

#[derive(Default)]
enum NetworkIdState {
    #[default]
    Collecting,
    Received(NetworkId),
    Error(String),
}

pub enum Message {
    CollectNetworkId,
    NetworkIdReceived(NetworkId),
    Error(String),
}

#[derive(Properties, PartialEq)]
pub struct WalletNetworkIdProperties {
    pub wallet: ConnectedWallet,
}

impl Component for WalletNetworkId {
    type Message = Message;
    type Properties = WalletNetworkIdProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Message::CollectNetworkId);

        Self::default()
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().ne(old_props) {
            ctx.link().send_message(Message::CollectNetworkId);
            true
        } else {
            false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::CollectNetworkId => {
                let link = ctx.link().clone();
                let wallet = ctx.props().wallet.clone();

                spawn_local(async move {
                    match wallet.network_id().await {
                        Ok(network_id) => {
                            link.send_message(Message::NetworkIdReceived(network_id));
                        }
                        Err(error) => {
                            link.send_message(Message::Error(error.to_string()));
                        }
                    }
                });

                let prev = std::mem::replace(&mut self.state, NetworkIdState::Collecting);

                !matches!(prev, NetworkIdState::Collecting)
            }
            Message::NetworkIdReceived(network_id) => {
                self.state = NetworkIdState::Received(network_id);
                true
            }
            Message::Error(error) => {
                self.state = NetworkIdState::Error(error);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style;

        let state = match &self.state {
            NetworkIdState::Collecting => {
                style = "text-bg-secondary";
                html! {
                    <div class={classes!("spinner-border", "text-primary", "me-2")} role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                }
            }
            NetworkIdState::Received(network_id) => {
                style = "text-bg-primary";
                html!({ format!("{network_id}") })
            }
            NetworkIdState::Error(error) => {
                style = "text-bg-warning";
                gloo::console::error!(format!("{error}"));

                html!(<></>)
            }
        };

        html! {
            <span class={classes!("badge", "rounded-pill", style, "mx-2")}>
                {state}
            </span>
        }
    }
}
