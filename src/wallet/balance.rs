use cardano_connector::ConnectedWallet;
use yew::{platform::spawn_local, prelude::*};

#[derive(Default)]
pub struct WalletBalance {
    state: BalanceState,
}

#[derive(Default)]
enum BalanceState {
    #[default]
    Loading,
    Loaded(u64),
    Error(String),
}

pub enum Message {
    CollectBalance,
    BalanceCollected(u64),
    Error(String),
}

#[derive(Properties, PartialEq)]
pub struct WalletBalanceProperties {
    pub wallet: ConnectedWallet,
}

impl Component for WalletBalance {
    type Message = Message;
    type Properties = WalletBalanceProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Message::CollectBalance);

        Self::default()
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().ne(old_props) {
            ctx.link().send_message(Message::CollectBalance);
            true
        } else {
            false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::CollectBalance => {
                let link = ctx.link().clone();
                let wallet = ctx.props().wallet.clone();

                spawn_local(async move {
                    match wallet.balance().await {
                        Ok(balance) => {
                            link.send_message(Message::BalanceCollected(balance));
                        }
                        Err(error) => {
                            link.send_message(Message::Error(error.to_string()));
                        }
                    }
                });

                let prev = std::mem::replace(&mut self.state, BalanceState::Loading);

                !matches!(prev, BalanceState::Loading)
            }
            Message::BalanceCollected(balance) => {
                self.state = BalanceState::Loaded(balance);
                true
            }
            Message::Error(error) => {
                self.state = BalanceState::Error(error);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let balance_state = match &self.state {
            BalanceState::Loading => {
                html! {
                    <>
                        <div class={classes!("spinner-border", "text-primary", "me-2")} role="status">
                            <span class="visually-hidden">{"Loading..."}</span>
                        </div>
                        <span class="text-muted">{"Loading balance..."}</span>
                    </>
                }
            }
            BalanceState::Loaded(balance) => {
                let balance_ada = format!("{:.2}", *balance as f64 / 1_000_000.0);
                let balance_lovelace = format!("{}", balance);
                html! {
                    <>
                    <div class={classes!("fs-4", "fw-bold")}>
                        {format!("{balance_ada} ₳")}
                    </div>
                    <div class={classes!("text-muted", "small")}>
                        {format!("{} lovelace", balance_lovelace)}
                    </div>
                    </>
                }
            }
            BalanceState::Error(error) => {
                html! {
                    <>
                        <div class={classes!("fs-4", "fw-bold")}>
                            {format!("Error: {}", error)}
                        </div>
                    </>
                }
            }
        };

        html! {
            <div class={classes!("d-flex", "flex-column", "align-items-start", "balance-display")}>
                {balance_state}
            </div>
        }
    }
}
