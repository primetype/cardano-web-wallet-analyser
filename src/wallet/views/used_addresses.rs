use super::{ReportStatus, ReportView};
use cardano_connector::{Address, Utxo};
use std::{borrow::Cow, collections::HashMap};
use yew::prelude::*;

#[derive(Default)]
pub struct UsedAddressesView;

#[derive(PartialEq, Properties)]
pub struct Properties {
    pub utxos: Option<HashMap<Address, Vec<Utxo>>>,
}

pub type Message = ();

impl Component for UsedAddressesView {
    type Message = Message;
    type Properties = Properties;

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
                    msg: Cow::Borrowed("Searching for UTxOs with used Addresses..."),
                };
                children = html!();
            }
            Some(utxos) if utxos.is_empty() => {
                status = ReportStatus::Success {
                    msg: Cow::Borrowed("You don't have UTxOs with used Addresses."),
                };
                children = html!();
            }
            Some(utxos) => {
                let num_used_addresses = utxos.len();
                let num_utxos = utxos.iter().fold(0, |count, (_, vec)| count + vec.len());

                status = ReportStatus::Warning {
                    msg: format!(
                        "You have {num_used_addresses} used Addresses across {num_utxos} UTxOs.."
                    )
                    .into(),
                };
                children = html! { <> {"TODO"} </> };
            }
        }

        html! {
            <ReportView id="used-addresses" status={status}>
                {children}
            </ReportView>
        }
    }
}
