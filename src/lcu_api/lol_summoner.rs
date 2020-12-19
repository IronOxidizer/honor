
use anyhow::Result;
use serde::Deserialize;
use super::super::util::*;
#[derive(Debug, Deserialize, Clone)]
pub struct Summoner {
    accountId: u32,
    pub displayName: String,
    internalName: String,
    nameChangeFlag: bool,
    percentCompleteForNextLevel: u8,
    profileIconId: u32,
    puuid: String,
    rerollPoints: RerollPoints,
    summonerId:u32,
    summonerLevel: u32,
    unnamed: bool,
    xpSinceLastLevel: u32,
    xpUntilNextLevel: u32
}

#[derive(Debug, Deserialize, Clone)]
pub struct RerollPoints {
    currentPoints: u32,
    maxRolls: u32,
    numberOfRolls: u32,
    pointsCostToRoll: u32,
    pointsToReroll: u32
}

pub async fn current_summoner(connection_data: ConnectionData) ->  Result<Summoner> {
    get_request(connection_data, "lol-summoner/v1/current-summoner").await
}

