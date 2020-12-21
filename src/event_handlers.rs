use std::sync::Arc;
use druid::{ExtEventSink, Selector, Target, SingleUse};
use anyhow::Result;

use super::util::*;
use super::lcu_api::*;

// These are all very similar, maybe make generic and only require endpoint.
pub const SET_CURRENT_SUMMONER: Selector<SingleUse<lol_summoner::Summoner>> = Selector::new("SET_CURRENT_SUMMONER");
pub async fn update_current_summoner(event_sink: Arc<ExtEventSink>, connection_data: Connection) -> Result<()> {
    let current_summoner = lol_summoner::current_summoner(connection_data).await?;
    event_sink.submit_command(
        SET_CURRENT_SUMMONER,
        SingleUse::new(current_summoner),
        Target::Auto)?;
    Ok(())
}

pub const SET_QUEUES: Selector<SingleUse<lol_game_queues::Queues>> = Selector::new("SET_QUEUES");
pub async fn update_queues(event_sink: Arc<ExtEventSink>, connection_data: Connection) -> Result<()> {
    let queues = lol_game_queues::queues(connection_data).await?;
    event_sink.submit_command(
        SET_QUEUES,
        SingleUse::new(queues),
        Target::Auto)?;
    Ok(())
}