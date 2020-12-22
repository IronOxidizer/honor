use anyhow::Result;
use serde::Deserialize;
use druid::{Data, Lens};
use druid::im::Vector;

use super::super::util::*;

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Queues {
    pub ranked: Vector<Queue>,
    pub casual: Vector<Queue>,
    pub versus_ai: Vector<Queue>
}

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Queue {
    pub description: String
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct _Queue {
    allowablePremadeSizes: Vec<u8>,
    areFreeChampionsAllowed: bool,
    assetMutator: String,
    category: String,
    championsRequiredToPlay: u8,
    description: String,
    detailedDescription: String,
    gameMode: String,
    gameTypeConfig: _GameTypeConfig,
    id: u32,
    isRanked: bool,
    isTeamBuilderManaged: bool,
    isTeamOnly: bool,
    lastToggledOffTime: u64,
    lastToggledOnTime: u64,
    mapId: u8,
    maxDivisionForPremadeSize2: String,
    maxLevel: u8,
    maxSummonerLevelForFirstWinOfTheDay: u8,
    maxTierForPremadeSize2: String,
    maximumParticipantListSize: u8,
    minLevel: u8,
    minimumParticipantListSize: u8,
    name: String,
    numPlayersPerTeam: u8,
    queueAvailability: String,
    queueRewards: _QueueRewards,
    removalFromGameAllowed: bool,
    removalFromGameDelayMinutes: u32,
    shortName: String,
    showPositionSelector: bool,
    spectatorEnabled: bool,
    r#type: String
}

impl _Queue {
    fn to_data(self) -> Queue {
        Queue {
            description: self.description,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct _GameTypeConfig {
    advancedLearningQuests: bool,
    allowTrades: bool,
    banMode: String,
    banTimerDuration: u32,
    battleBoost: bool,
    crossTeamChampionPool: bool,
    deathMatch: bool,
    doNotRemove: bool,
    duplicatePick: bool,
    exclusivePick: bool,
    gameModeOverride: Option<bool>,
    id: u32,
    learningQuests: bool,
    mainPickTimerDuration: u32,
    maxAllowableBans: u8,
    name: String,
    numPlayersPerTeamOverride: Option<u8>,
    onboardCoopBeginner: bool,
    pickMode: String,
    postPickTimerDuration: u32,
    reroll: bool,
    teamChampionPool: bool
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct _QueueRewards {
    isChampionPointsEnabled: bool,
    isIpEnabled: bool,
    isXpEnabled: bool,
    partySizeIpRewards: Vec<u32>
}

pub async fn queues(connection_data: Connection) -> Result<Queues> {
    let mut available = get_request::<Vec<_Queue>>(connection_data, "lol-game-queues/v1/queues").await?;
    available.retain(|q| q.queueAvailability == "Available");
    available.sort_by(|a, b| a.description.cmp(&b.description));
    let avail_iter = available.into_iter();

    let ranked = avail_iter.clone()
        .filter_map(|q| if q.isRanked && !q.description.contains("Clash") {Some(q.to_data())} else {None}).collect();
    let versus_ai = avail_iter.clone()
        .filter_map(|q| if q.category == "VersusAi" || q.description.contains("Tutorial")
        {Some(q.to_data())} else {None}).collect();
    let casual = avail_iter
        .filter_map(|q| if !q.isRanked && q.category != "VersusAi" && !q.description.contains("Tutorial")
        {Some(q.to_data())} else {None}).collect();

    Ok(Queues {
        ranked: ranked,
        casual: casual,
        versus_ai: versus_ai
    })
}