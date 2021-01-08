use serde::Deserialize;
use std::sync::Arc;
use druid::{Data, Lens, ExtEventSink, SingleUse, Target,
    im::Vector}; // All im types are wrapped with Arc thus are cheap to clone

use super::*;

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Queues {
    pub ranked: Vector<Queue>,
    pub casual: Vector<Queue>,
    pub versus_ai: Vector<Queue>
}

impl From<Vec<Queue>> for Queues {
    fn from(mut queue_vec: Vec<Queue>) -> Self {
        queue_vec.retain(|q| q.queueAvailability == "Available");

        queue_vec.iter_mut().for_each(|q| {
            let gs = q.gameMode.as_str();
            if gs == "NEXUSBLITZ" {
                q.description = "Nexus Blitz".to_string();
            } else if gs != "CLASSIC" && gs != "ARAM" && !gs.starts_with("TUTORIAL") {
                q.description = format!("{}: {}", q.gameMode, q.description);
            }
        });

        queue_vec.sort_by(|a, b| a.description.cmp(&b.description));
        let queue_iter = queue_vec.into_iter();

        let ranked = queue_iter.clone().filter(|q|
            q.isRanked && !q.description.contains("Clash")).collect();
        let versus_ai = queue_iter.clone().filter(|q|
            q.category == "VersusAi" || q.description.contains("Tutorial")).collect();
        let casual = queue_iter.filter(|q|
            !q.isRanked && q.category != "VersusAi" && !q.description.contains("Tutorial")).collect();

        Self {
            ranked,
            casual,
            versus_ai
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Data)]
pub struct Queue {
    // allowablePremadeSizes: Vec<u8>,
    // areFreeChampionsAllowed: bool,
    // assetMutator: String,
    category: String,
    // championsRequiredToPlay: u8,
    pub description: String,
    // detailedDescription: String,
    gameMode: String,
    // gameTypeConfig: _GameTypeConfig,
    // id: u32,
    isRanked: bool,
    // isTeamBuilderManaged: bool,
    // isTeamOnly: bool,
    // lastToggledOffTime: u64,
    // lastToggledOnTime: u64,
    // mapId: u8,
    // maxDivisionForPremadeSize2: String,
    // maxLevel: u8,
    // maxSummonerLevelForFirstWinOfTheDay: u8,
    // maxTierForPremadeSize2: String,
    // maximumParticipantListSize: u8,
    // minLevel: u8,
    // minimumParticipantListSize: u8,
    // name: String,
    // numPlayersPerTeam: u8,
    queueAvailability: String,
    // queueRewards: _QueueRewards,
    // removalFromGameAllowed: bool,
    // removalFromGameDelayMinutes: u32,
    // shortName: String,
    // showPositionSelector: bool,
    // spectatorEnabled: bool,
    // r#type: String
}

// #[allow(non_snake_case)]
// #[derive(Clone, Debug, Deserialize)]
// struct _GameTypeConfig {
//     advancedLearningQuests: bool,
//     allowTrades: bool,
//     banMode: String,
//     banTimerDuration: u32,
//     battleBoost: bool,
//     crossTeamChampionPool: bool,
//     deathMatch: bool,
//     doNotRemove: bool,
//     duplicatePick: bool,
//     exclusivePick: bool,
//     gameModeOverride: Option<bool>,
//     id: u32,
//     learningQuests: bool,
//     mainPickTimerDuration: u32,
//     maxAllowableBans: u8,
//     name: String,
//     numPlayersPerTeamOverride: Option<u8>,
//     onboardCoopBeginner: bool,
//     pickMode: String,
//     postPickTimerDuration: u32,
//     reroll: bool,
//     teamChampionPool: bool
// }

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct _QueueRewards {
    isChampionPointsEnabled: bool,
    isIpEnabled: bool,
    isXpEnabled: bool,
    partySizeIpRewards: Vec<u32>
}

pub fn get_queues(http_connection: HttpConnection, event_sink: Arc<ExtEventSink>) {
    tokio::spawn(async move {
        let queues = Arc::new(Queues::from(
            get_request::<Vec<Queue>>(http_connection, "lol-game-queues/v1/queues").await.unwrap()
        ));

        event_sink.submit_command(
            SET_QUEUES,
            SingleUse::new(queues),
            Target::Auto).unwrap();
    });
}