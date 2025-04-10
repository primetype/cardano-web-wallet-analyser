use cardano_connector::Wallet;
use std::rc::Rc;
use yew::{prelude::*, Reducible, UseReducerHandle};

pub enum Action {
    WalletSelected(Wallet),
}

#[derive(PartialEq)]
pub struct Context {
    // once a wallet is selected this will be set.
    pub wallet: Option<Wallet>,
}

pub type ContextHandle = UseReducerHandle<Context>;

impl Context {
    pub fn new() -> Self {
        crate::style::initialize_theme();
        Self { wallet: None }
    }

    fn reset() -> Self {
        Self { wallet: None }
    }
}

impl Reducible for Context {
    type Action = Action;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::WalletSelected(wallet) => {
                let toggle = self
                    .wallet
                    .as_ref()
                    .map(|old| old.icon() == wallet.icon())
                    .unwrap_or_default();

                if toggle {
                    Rc::new(Self::reset())
                } else {
                    Rc::new(Self {
                        wallet: Some(wallet),
                        ..Self::reset()
                    })
                }
            }
        }
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ContextProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AppContextProvider(props: &ContextProviderProps) -> Html {
    let msg = use_reducer(|| Context::new());

    html! {
        <ContextProvider<ContextHandle> context={msg}>
            {props.children.clone()}
        </ContextProvider<ContextHandle>>
    }
}
