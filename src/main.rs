use cardano_web_wallet_analyser::{icons, AppBody, AppContextProvider, WalletList};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <AppContextProvider>
        <nav class="navbar navbar-expand-lg navbar-dark bg-primary mb-4">
            <div class="container">
                <a class="navbar-brand" href="#">
                    {"Cardano Wallet Analyser"}
                </a>
                <div class="navbar-nav ms-auto">
                    <a
                        class="nav-link"
                        href="https://github.com/primetype/cardano-analyser"
                        rel="external noreferrer"
                        target="_blank">
                    <icons::Github />
                    {" GitHub"}
                    </a>
                </div>
            </div>
        </nav>

            <div class="container my-4">
                <div class="row d-flex">
                    <WalletList />
                </div>
                <AppBody />
            </div>
        </AppContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
