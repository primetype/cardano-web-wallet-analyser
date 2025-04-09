use crate::{context::ContextHandle, wallet::Wallet};
use yew::prelude::*;

#[function_component(AppBody)]
pub fn body() -> Html {
    let ctx = use_context::<ContextHandle>().unwrap();

    if let Some(wallet) = ctx.wallet.as_ref() {
        // Handle wallet-related logic here
        html! { < Wallet wallet={wallet.clone()} /> }
    } else {
        html! {
            <>
                <div class="row">
                    <div class="col-lg-12">
                        <div class="card bg-light mb-4">
                            <div class="card-body">
                                <div class="d-flex align-items-center">
                                    <div>
                                        <h2 class="mb-1">{"Cardano Wallet UTxO Analyzer"}</h2>
                                        <p class="mb-0">{"Analyze your wallet's UTxOs for security risks and optimization opportunities"}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="row mb-4">
                    <div class="col-md-4">
                        <div class="card h-100">
                            <div class="card-body">
                                <h5 class="card-title"><i class="bi bi-shield-check me-2"></i>{"UTxO Security Analysis"}</h5>
                                <p class="card-text">{"Analyzes your UTxOs and identifies potential security risks like exposed public keys."}</p>
                                <span class="badge bg-primary">{"Security Scanning"}</span>
                            </div>
                        </div>
                    </div>

                    <div class="col-md-4">
                        <div class="card h-100">
                            <div class="card-body">
                                <h5 class="card-title"><i class="bi bi-key me-2"></i>{"Stake Key Verification"}</h5>
                                <p class="card-text">{"Detect UTxOs linked to external stake keys and maintain better wallet hygiene."}</p>
                                <span class="badge bg-success">{"Automated Checks"}</span>
                            </div>
                        </div>
                    </div>

                    <div class="col-md-4">
                        <div class="card h-100">
                            <div class="card-body">
                                <h5 class="card-title"><i class="bi bi-coin me-2"></i>{"UTxO Management"}</h5>
                                <p class="card-text">{"Identify UTxOs when to consolidate UTxOs."}</p>
                                <span class="badge bg-warning">{"Performance Optimization"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
