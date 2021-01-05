use serde::Deserialize;
use std::sync::Arc;

use druid::{Data, Lens, ExtEventSink, Selector, SingleUse, Target,
    im::{Vector}};

use super::*;

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Queues {
    pub ranked: Vector<Queue>,
    pub casual: Vector<Queue>,
    pub versus_ai: Vector<Queue>
}

impl From<Vec<_Queue>> for Queues {
    fn from(mut queue_vec: Vec<_Queue>) -> Self {
        queue_vec.retain(|q| q.queueAvailability == "Available");
        queue_vec.sort_by(|a, b| a.description.cmp(&b.description));
        let queue_iter = queue_vec.into_iter();

        let ranked = queue_iter.clone()
            .filter_map(|q| if q.isRanked && !q.description.contains("Clash") {Some(q.to_data())} else {None}).collect();
        let versus_ai = queue_iter.clone()
            .filter_map(|q| if q.category == "VersusAi" || q.description.contains("Tutorial")
            {Some(q.to_data())} else {None}).collect();
        let casual = queue_iter
            .filter_map(|q| if !q.isRanked && q.category != "VersusAi" && !q.description.contains("Tutorial")
            {Some(q.to_data())} else {None}).collect();

        Self {
            ranked: ranked,
            casual: casual,
            versus_ai: versus_ai
        }
    }
}

#[derive(Clone, Debug, Data, Lens)]
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

pub const SET_QUEUES: Selector<SingleUse<Arc<Queues>>> = Selector::new("SET_QUEUES");
pub fn queues(http_connection: HttpConnection, event_sink: Arc<ExtEventSink>) {
    tokio::spawn(async move {
        let queues = Arc::new(Queues::from(
            get_request::<Vec<_Queue>>(http_connection, "lol-game-queues/v1/queues").await.unwrap()
        ));

        event_sink.submit_command(
            SET_QUEUES,
            SingleUse::new(queues),
            Target::Auto).unwrap();
    });
}