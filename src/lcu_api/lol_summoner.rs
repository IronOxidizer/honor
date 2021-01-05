use std::sync::Arc;
use anyhow::Result;
use serde::Deserialize;
use druid::{Data, Lens, ExtEventSink, Selector, SingleUse, Target};

use super::*;

// Need to shadow _Summoner until Deserialize is implement for druid::im::Vector
#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Summoner {
    pub display_name: String
}

impl From<_Summoner> for Summoner {
    fn from(summoner: _Summoner) -> Self {
        Self {
            display_name: summoner.displayName
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Default, Debug, Deserialize)]
struct _Summoner {
    accountId: u32,
    displayName: String,
    internalName: String,
    nameChangeFlag: bool,
    percentCompleteForNextLevel: u8,
    profileIconId: u32,
    puuid: String,
    rerollPoints: _RerollPoints,
    summonerId:u32,
    summonerLevel: u32,
    unnamed: bool,
    xpSinceLastLevel: u32,
    xpUntilNextLevel: u32
}

#[allow(non_snake_case)]
#[derive(Clone, Default, Debug, Deserialize)]
struct _RerollPoints {
    currentPoints: u32,
    maxRolls: u32,
    numberOfRolls: u32,
    pointsCostToRoll: u32,
    pointsToReroll: u32
}

pub const SET_CURRENT_SUMMONER: Selector<SingleUse<Arc<lol_summoner::Summoner>>> = Selector::new("SET_CURRENT_SUMMONER");
pub async fn current_summoner(http_connection: HttpConnection, event_sink: Arc<ExtEventSink>) -> Result<()> {
    let summoner = Arc::new(Summoner::from(
        get_request::<_Summoner>(http_connection, "lol-summoner/v1/current-summoner").await?
    ));
    event_sink.submit_command(
        SET_CURRENT_SUMMONER,
        SingleUse::new(summoner),
        Target::Auto)?;
    Ok(())
}