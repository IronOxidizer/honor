use serde::Serialize;
use druid::im::Vector;

use super::*;

#[allow(non_snake_case)]
#[derive(Clone, Default, Debug, Deserialize, Data, Lens)]
pub struct Lobby {
    // canStartActivity: bool,
    // chatRoomId: String,
    // chatRoomKey: String,
    // gameConfig: GameConfig,
    // invitations: Vector<Invite>, // Causes a lifetime error
    // localMember: LobbyMember,
    pub members: Vector<LobbyMember>,
    // partyId: String,
    // partyType: String,
    // restrictions: Vector<String>,
    // warnings: Vector<String>
}

// #[allow(non_snake_case)]
// #[derive(Clone, Default, Debug, Deserialize, Data, Lens)]
// struct GameConfig {
//     allowablePremadeSizes: Vector<u8>
// }

#[allow(non_snake_case)]
#[derive(Clone, Default, Debug, Deserialize, Data, Lens)]
pub struct LobbyMember {
    // allowedChangeActivity: bool,
    // allowedInviteOthers: bool,
    // allowedKickOthers: bool,
    // allowedStartActivity: bool,
    // allowedToggleInvite: bool,
    // autoFillEligible: bool,
    // autoFillProtectedForPromos: bool,
    // autoFillProtectedForSoloing: bool,
    // autoFillProtectedForStreaking: bool,
    // botChampionId: u32,
    // botDifficulty: String,
    // botId: String,
    pub firstPositionPreference: String,
    // isBot: bool,
    // isLeader: bool,
    // isSpectator: bool,
    // puuid: String,
    // ready: bool,
    pub secondPositionPreference: String,
    // showGhostedBanner: bool,
    // summonerIconId: u32,
    // summonerId: u32,
    // summonerInternalName: String,
    // summonerLevel: u32,
    pub summonerName: String,
    // teamId: u8
}

#[allow(non_snake_case)]
#[derive(Clone, Default, Debug, Serialize, Data, Lens)]
pub struct Invite {
    invitationId: String,
    state: &'static str,
    timestamp: String,
    pub toSummonerId: u64,
    toSummonerName: String
}

#[allow(non_snake_case)]
impl Invite {
    pub fn new(toSummonerId: u64) -> Self {
        Self {
            state: "Requested",
            toSummonerId,
            ..Default::default()
        }
    }
}

pub fn post_lobby_invitations(http_connection: HttpConnection, invite: Invite) {
    tokio::spawn(async move {
        post_request(http_connection, "lol-lobby/v2/lobby/invitations", 
            serde_json::to_string(&vec!(invite)).unwrap()).await.unwrap();
        //eprintln!("{:?}", res);
    });
}
