pub mod byron_address;
pub mod inconsistent_reward_addresses;
pub mod used_addresses;

use crate::icons::{Bug, CheckMark, Warning};
use std::borrow::Cow;

pub use self::{
    byron_address::ByronAddressView,
    inconsistent_reward_addresses::InconsistentRewardAddressesView,
    used_addresses::UsedAddressesView,
};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Pending { msg: Cow<'static, str> },
    Success { msg: Cow<'static, str> },
    Warning { msg: Cow<'static, str> },
    Error { msg: Cow<'static, str> },
}

#[derive(Properties, PartialEq)]
pub(self) struct ReportProperties {
    pub id: &'static str,
    pub status: ReportStatus,
    pub children: Html,
}

#[derive(Default)]
pub(self) struct ReportView;

impl Component for ReportView {
    type Message = ();
    type Properties = ReportProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ReportProperties {
            status,
            children,
            id,
        } = ctx.props();

        let id = format!("{id}-accordion");

        let header: Html = match status {
            ReportStatus::Pending { msg } => {
                html! {
                    <button class="accordion-button collapsed" type="button" disabled=true>
                        <div class="spinner-border text-primary me-2" role="status">
                            <span class="visually-hidden">{"Loading..."}</span>
                        </div>
                        <span>{msg.to_owned()}</span>
                    </button>
                }
            }
            ReportStatus::Success { msg } => {
                html! {
                    <button class="accordion-button collapsed" type="button" disabled=true>
                        <span class="text-success me-3">
                            <CheckMark />
                        </span>
                        {msg.to_owned()}
                    </button>
                }
            }
            ReportStatus::Warning { msg } => {
                html! {
                    <button class="accordion-button collapsed text-error" type="button" data-bs-toggle="collapse" data-bs-target={format!("#{id}")}>
                        <span class="text-warning me-3">
                            <Warning />
                        </span>
                        {msg.to_owned()}
                    </button>
                }
            }
            ReportStatus::Error { msg } => {
                html! {
                    <button class="accordion-button collapsed text-error" type="button" data-bs-toggle="collapse" data-bs-target={format!("#{id}")}>
                        <span class="text-error me-3">
                            <Bug />
                        </span>
                        {msg.to_owned()}
                    </button>
                }
            }
        };

        html! {
        <div class="accordion my-2" id="byron_address_view">
            <div class="accordion-item">
                <h2 class="accordion-header">
                    {header}
                </h2>
                <div id={id} class="accordion-collapse collapse">
                    <div class="accordion-body">
                        <div class="table-responsive">
                            {children}
                        </div>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
