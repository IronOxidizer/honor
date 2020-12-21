use anyhow::Result;
use serde::Deserialize;
use druid::{Data, Lens};

use super::super::util::*;

// Need to shadow _Summoner until Deserialize is implement for druid::im::Vector
#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Summoner {
    pub display_name: String
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

impl _Summoner {
    fn to_data(self) -> Summoner {
        Summoner {
            display_name: self.displayName
        }
    }
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

pub async fn current_summoner(connection: Connection) ->  Result<Summoner> {
    Ok(get_request::<_Summoner>(connection, "lol-summoner/v1/current-summoner")
        .await?.to_data())
}