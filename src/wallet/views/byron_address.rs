use super::{ReportStatus, ReportView};
use cardano_connector::{Address, Utxo};
use std::{borrow::Cow, collections::HashMap};
use yew::prelude::*;

#[derive(Default)]
pub struct ByronAddressView;

#[derive(PartialEq, Properties)]
pub struct ByronAddressProperties {
    pub utxos: Option<HashMap<Address, Vec<Utxo>>>,
}

pub type Message = ();

impl Component for ByronAddressView {
    type Message = Message;
    type Properties = ByronAddressProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let utxos = &ctx.props().utxos;

        let status: ReportStatus;
        let children: Html;

        match utxos {
            None => {
                status = ReportStatus::Pending {
                    msg: Cow::Borrowed("Searching for Active Byron Addresses..."),
                };
                children = html!();
            }
            Some(utxos) if utxos.is_empty() => {
                status = ReportStatus::Success {
                    msg: Cow::Borrowed("You don't have active Byron Addresses."),
                };
                children = html!();
            }
            Some(utxos) => {
                let num_byron_addresses = utxos.len();
                let num_byron_utxos = utxos.iter().fold(0, |count, (_, vec)| count + vec.len());

                status = ReportStatus::Warning { msg: format!("You still have {num_byron_addresses} active Byron Addresses across {num_byron_utxos} UTxOs.").into() };
                children = html! { <> {"TODO"} </> };
            }
        }

        html! {
            <ReportView id="byron-utxos-addresses" status={status}>
                {children}
            </ReportView>
        }
    }
}
