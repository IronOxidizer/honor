use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;
use druid::{Data, Lens, ExtEventSink, Selector, SingleUse, Target,
    im::{Vector}};

use super::*;


// Seperate into 3 rows, League (different background or text color depending on status),
//      Green = Online, Blue = In-Game/Queue, Red = Away 
//      Other Game / Mobile, Offline

// Add group for in queue, turquois


#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Friends {
    pub online: Vector<Friend>,
    pub busy: Vector<Friend>,
    pub away: Vector<Friend>,
    pub other: Vector<Friend>,
    pub offline: Vector<Friend>,
}

impl From<Vec<_Friend>> for Friends {
    fn from(_friend_vec: Vec<_Friend>) -> Self {
        let mut friends = Self::default();
        for friend in _friend_vec.into_iter() {
            match friend.availability.as_str() {
                "chat" => friends.online.push_back(friend.into()),
                "dnd" => friends.busy.push_back(friend.into()),
                "away" => friends.away.push_back(friend.into()),
                "offline" => friends.offline.push_back(friend.into()),
                _ => friends.other.push_back(friend.into())
            }
        }
        friends
    }
}

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Friend {
    pub id: String,
    pub name: String
}

impl From<_Friend> for Friend {
    fn from(_friend: _Friend) -> Self {
        Self {
            id: _friend.id,
            name: if _friend.name.is_empty() {
                    _friend.gameName
                } else {
                    _friend.name
                }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
struct _Friend {
    availability: String,
    displayGroupId: u16,
    displayGroupName: String,
    gameName: String,
    gameTag: String,
    groupId: u16,
    groupName: String,
    icon: i16,
    id: String,
    isP2PConversationMuted: bool,
    lastSeenOnlineTimestamp: Option<u64>,
    // lol: String, // unknown object?
    name: String,
    note: String,
    patchline: String,
    pid: String,
    platformId: String,
    product: String,
    productName: String,
    puuid: String,
    statusMessage: String,
    summary: String,
    summonerId: u32,
    time: u64
}

pub const SET_FRIENDS: Selector<SingleUse<Arc<Friends>>> = Selector::new("SET_FRIENDS");
pub async fn friends( http_connection: HttpConnection, event_sink: Arc<ExtEventSink>) -> Result<()> {
    let friends = Arc::new(Friends::from(
        get_request::<Vec<_Friend>>(http_connection, "lol-chat/v1/friends").await.expect("Something went wrong here")
    ));
    event_sink.submit_command(
        SET_FRIENDS,
        SingleUse::new(friends),
        Target::Auto)?;
    Ok(())
}