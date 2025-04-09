use cardano_connector::{ConnectedWallet, Utxo};
use yew::{platform::spawn_local, prelude::*};

#[derive(Default)]
pub struct UtxosView {
    state: State,
}

#[derive(Default)]
enum State {
    #[default]
    Loading,
    Loaded(Vec<Utxo>),
    Error(String),
}

pub enum Message {
    CollectUtxos,
    UtxoCollected(Vec<Utxo>),
    Error(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct UtxosProperties {
    pub wallet: ConnectedWallet,
}

impl Component for UtxosView {
    type Message = Message;
    type Properties = UtxosProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Message::CollectUtxos);

        UtxosView::default()
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().ne(old_props) {
            ctx.link().send_message(Message::CollectUtxos);
            true
        } else {
            false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::CollectUtxos => {
                let link = ctx.link().clone();
                let wallet = ctx.props().wallet.clone();

                spawn_local(async move {
                    match wallet.all_utxos(None).await {
                        Ok(utxos) => link.send_message(Message::UtxoCollected(utxos)),
                        Err(error) => link.send_message(Message::Error(error.to_string())),
                    }
                });

                let prev = std::mem::replace(&mut self.state, State::Loading);

                !matches!(prev, State::Loading)
            }
            Message::UtxoCollected(utxos) => {
                self.state = State::Loaded(utxos);
                true
            }
            Message::Error(error) => {
                self.state = State::Error(error);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.state {
            State::Loading => html! {
                <div class="d-flex align-items-center mt-3">
                    <div class="spinner-border text-primary me-2" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                    <span>{"Loading UTXOs..."}</span>
                </div>
            },
            State::Loaded(utxos) => {
                let total_utxos = utxos.len();
                let total_value: u64 = utxos.iter().map(|utxo| utxo.amount()).sum();
                let total_value_ada = total_value as f64 / 1_000_000.0;

                html! {
                    <div class="mt-4">
                        <h4 class="mb-3">{"UTXOs Analysis"}</h4>
                        <div class="d-flex justify-content-between mb-3">
                            <div class="card bg-light me-2" style="flex: 1">
                                <div class="card-body">
                                    <h5 class="card-title">{"Total UTXOs"}</h5>
                                    <p class="card-text fs-3">{total_utxos}</p>
                                </div>
                            </div>
                            <div class="card bg-light" style="flex: 1">
                                <div class="card-body">
                                    <h5 class="card-title">{"Total Value"}</h5>
                                    <p class="card-text fs-3">{format!("{:.2} ₳", total_value_ada)}</p>
                                </div>
                            </div>
                        </div>
                        <div class="accordion" id="utxoAccordion">
                            <div class="accordion-item">
                                <h2 class="accordion-header">
                                    <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#utxoList">
                                        {"UTXO Details"}
                                    </button>
                                </h2>
                                <div id="utxoList" class="accordion-collapse collapse">
                                    <div class="accordion-body">
                                        <div class="table-responsive">
                                            <table class="table table-hover">
                                                <thead>
                                                    <tr>
                                                        <th>{"TX Hash"}</th>
                                                        <th>{"Output Index"}</th>
                                                        <th>{"Amount"}</th>
                                                        <th>{"Address"}</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    { for utxos.iter().map(|utxo| {
                                                        let tx_hash = utxo.transaction_id().to_string();
                                                        let output_index = utxo.index();
                                                        let amount = utxo.amount() as f64 / 1_000_000.0;
                                                        let address = &utxo.address().unwrap().to_bech32().unwrap();

                                                        html! {
                                                            <tr>
                                                                <td class="text-truncate" style="max-width: 150px;">{tx_hash}</td>
                                                                <td>{output_index}</td>
                                                                <td>{format!("{:.6} ₳", amount)}</td>
                                                                <td class="text-truncate" style="max-width: 250px;">{address}</td>
                                                            </tr>
                                                        }
                                                    }) }
                                                </tbody>
                                            </table>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                    </div>
                }
            }
            State::Error(error) => html! {
                <div class="alert alert-danger mt-3" role="alert">
                    <h5 class="alert-heading">{"Error Loading UTXOs"}</h5>
                    <p>{format!("{}", error)}</p>
                </div>
            },
        }
    }
}
