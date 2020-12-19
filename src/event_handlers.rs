use std::sync::Arc;
use druid::{ExtEventSink, Selector, Target, SingleUse};
use anyhow::Result;

use super::util::*;
use super::lcu_api;

// Consider changing to set_summoner, don't make single use and update multiple values / event handlers
//      with a single update

pub const SET_SUMMONER_NAME: Selector<SingleUse<String>> = Selector::new("event-example.set-color");

pub async fn update_summoner_name(connection_data: ConnectionData, event_sink: Arc<ExtEventSink>) -> Result<()> {
    let current_summoner = lcu_api::lol_summoner::current_summoner(connection_data).await?;
    event_sink.submit_command(
        SET_SUMMONER_NAME,
        SingleUse::new(current_summoner.displayName),
        Target::Auto)?;
    Ok(())
}