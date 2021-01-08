use serde::Deserialize;
use std::sync::Arc;
use druid::{Data, Lens, ExtEventSink, Selector, SingleUse, Target,
    im::Vector}; // All im types are wrapped with Arc thus are cheap to clone

use super::*;

/*
[
  {
    "invitationId": "string",
    "state": "Requested",
    "timestamp": "string",
    "toSummonerId": 0,
    "toSummonerName": "string"
  }
]
*/
pub fn post_lobby_invitations(http_connection: HttpConnection, summonerId: u32) {
    tokio::spawn(async move {
        let res = post_request(http_connection, "lol-game-queues/v1/queues",
        format!("[{{\"toSummonerId\":{}}}]", summonerId)
        ).await.unwrap();
        eprintln!("{:?}", res);
    });
}