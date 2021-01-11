use serde::Serialize;

use super::*;

/*
#[allow(non_snake_case)]
pub struct Lobby {
    canStartActivity: bool,
    chatRoomId: String,
    chatRoomKey: String,
    gameConfig: GameConfig,
    invitations: Vec<Invite>,
    localMember: LobbyMember,
    members: Vec<LobbyMember>,
    partyId: String,
    partyType: String,
    restrictions: Vec<String>,
    warnings: Vec<String>
}

#[allow(non_snake_case)]
struct GameConfig {
    allowablePremadeSizes: Vec<u8>
}

struct LobbyMember {
    allowedChangeActivity: bool,
    allowedInviteOthers: bool,
    allowedKickOthers: bool,
    allowedStartActivity: bool,
    allowedToggleInvite: bool,
    autoFillEligible: bool,
    autoFillProtectedForPromos: bool,
    autoFillProtectedForSoloing: bool,
    autoFillProtectedForStreaking: bool,
    botChampionId: u32,
    botDifficulty: String,
    botId: u8,
    firstPositionPreference: String,
    isBot: bool,
    isLeader: bool,
    isSpectator: bool,
    puuid: String,
    ready: bool,
    secondPositionPreference: String,
    showGhostedBanner: bool,
    summonerIconId: u32,
    summonerId: u32,
    summonerInternalName: String,
    summonerLevel: u32,
    summonerName: String,
    teamId: u8
}
*/

#[allow(non_snake_case)]
#[derive(Clone, Default, Debug, Serialize)]
pub struct Invite {
    invitationId: String,
    state: &'static str,
    timestamp: String,
    pub toSummonerId: u32,
    toSummonerName: String
}

#[allow(non_snake_case)]
impl Invite {
    pub fn new(toSummonerId: u32) -> Self {
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
