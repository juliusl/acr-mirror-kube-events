use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::core::v1::Event;
use kube::{
    api::Api,
    runtime::{watcher, WatchStreamExt},
    Client,
};

use acr_mirror_kube_events::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await?;

    let events: Api<Event> = Api::all(client);
    let wc = watcher::Config::default();

    let ew = watcher(events, wc).applied_objects();

    piu_mut!(ew);
    while let Some(event) = eq.try_next().await? {
        handle_event(event)?;
    }

    Ok(())
}

fn handle_event(ev: Event) -> Result<()> {
    if let Some("Pulling") = ev.reason.map(|r| r.as_str()) {
        println!("{:?}", ev);
    }

    Ok(())
}