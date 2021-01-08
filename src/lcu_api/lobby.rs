use super::*;

pub fn post_lobby_invitations(http_connection: HttpConnection, summoner_id: u32) {
    tokio::spawn(async move {
        let res = post_request(http_connection, "lol-lobby/v2/lobby/invitations",
        format!("[
  {{
    \"invitationId\": \"\",
    \"state\": \"Requested\",
    \"timestamp\": \"\",
    \"toSummonerId\": {},
    \"toSummonerName\": \"\"
  }}
]", summoner_id)
        ).await.unwrap();
        eprintln!("{:?}", res);
    });
}

/*
{
  "canStartActivity": true,
  "chatRoomId": "6c116e58-c66d-411d-8377-c466b3346164",
  "chatRoomKey": "6c116e58-c66d-411d-8377-c466b3346164",
  "gameConfig": {
    "allowablePremadeSizes": [
      1,
      2,
      3,
      4,
      5
    ],
    "customLobbyName": "",
    "customMutatorName": "",
    "customRewardsDisabledReasons": [],
    "customSpectatorPolicy": "NotAllowed",
    "customSpectators": [],
    "customTeam100": [],
    "customTeam200": [],
    "gameMode": "CLASSIC",
    "isCustom": false,
    "isLobbyFull": false,
    "isTeamBuilderManaged": false,
    "mapId": 11,
    "maxHumanPlayers": 0,
    "maxLobbySize": 5,
    "maxTeamSize": 5,
    "pickType": "",
    "premadeSizeAllowed": true,
    "queueId": 430,
    "showPositionSelector": false
  },
  "invitations": [
    {
      "invitationId": "",
      "state": "Accepted",
      "timestamp": "0",
      "toSummonerId": 88392922,
      "toSummonerName": "Mackaron"
    }
  ],
  "localMember": {
    "allowedChangeActivity": true,
    "allowedInviteOthers": true,
    "allowedKickOthers": true,
    "allowedStartActivity": true,
    "allowedToggleInvite": true,
    "autoFillEligible": false,
    "autoFillProtectedForPromos": false,
    "autoFillProtectedForSoloing": false,
    "autoFillProtectedForStreaking": false,
    "botChampionId": 0,
    "botDifficulty": "NONE",
    "botId": "",
    "firstPositionPreference": "",
    "isBot": false,
    "isLeader": true,
    "isSpectator": false,
    "puuid": "010b2399-3d0f-597d-8b43-68e6187336dc",
    "ready": true,
    "secondPositionPreference": "",
    "showGhostedBanner": false,
    "summonerIconId": 29,
    "summonerId": 88392922,
    "summonerInternalName": "Mackaron",
    "summonerLevel": 143,
    "summonerName": "Mackaron",
    "teamId": 0
  },
  "members": [
    {
      "allowedChangeActivity": true,
      "allowedInviteOthers": true,
      "allowedKickOthers": true,
      "allowedStartActivity": true,
      "allowedToggleInvite": true,
      "autoFillEligible": false,
      "autoFillProtectedForPromos": false,
      "autoFillProtectedForSoloing": false,
      "autoFillProtectedForStreaking": false,
      "botChampionId": 0,
      "botDifficulty": "NONE",
      "botId": "",
      "firstPositionPreference": "",
      "isBot": false,
      "isLeader": true,
      "isSpectator": false,
      "puuid": "010b2399-3d0f-597d-8b43-68e6187336dc",
      "ready": true,
      "secondPositionPreference": "",
      "showGhostedBanner": false,
      "summonerIconId": 29,
      "summonerId": 88392922,
      "summonerInternalName": "Mackaron",
      "summonerLevel": 143,
      "summonerName": "Mackaron",
      "teamId": 0
    }
  ],
  "partyId": "6c116e58-c66d-411d-8377-c466b3346164",
  "partyType": "open",
  "restrictions": [],
  "warnings": []
}
*/