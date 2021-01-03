use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;
use druid::{Data, Lens};
use druid::im::{Vector};

use super::super::util::*;


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
                "chat" => friends.online.push_back(friend.to_owned().to_data()),
                "dnd" => friends.busy.push_back(friend.to_owned().to_data()),
                "away" => friends.away.push_back(friend.to_owned().to_data()),
                "offline" => friends.offline.push_back(friend.to_owned().to_data()),
                _ => friends.other.push_back(friend.to_owned().to_data())
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

impl _Friend {
    fn to_data(self) -> Friend {
        Friend {
            id: self.id,
            name: if self.name.is_empty() {self.gameName} else {self.name}
        }
    }
}

pub async fn friends(connection: Connection) ->  Result<Arc<Friends>> {
    Ok(Arc::new(Friends::from(
        get_request::<Vec<_Friend>>(connection, "lol-chat/v1/friends").await?
    )))
}