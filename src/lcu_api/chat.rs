use serde::Deserialize;
use std::sync::Arc;
use druid::{Data, Lens, ExtEventSink, SingleUse, Target,
    im::Vector}; // All im types are wrapped with Arc thus are cheap to clone

use super::*;

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Friends {
    // Add group for in_queue, turquois
    pub online: Vector<Arc<Friend>>,
    pub busy: Vector<Arc<Friend>>,
    pub away: Vector<Arc<Friend>>,
    pub other: Vector<Arc<Friend>>,
    pub offline: Vector<Arc<Friend>>,
}

impl Friends {
    fn insert(&mut self, mut friend: Friend) {
        if friend.name.is_empty() {
            friend.name = friend.gameName.clone()
        }

        match friend.availability.as_str() {
            "chat" => self.online.push_front(Arc::new(friend)),
            "dnd" => self.busy.push_front(Arc::new(friend)),
            "away" => self.away.push_front(Arc::new(friend)),
            "offline" => self.offline.push_front(Arc::new(friend)),
            _ => self.other.push_front(Arc::new(friend))
        };
    }

    pub fn update(&mut self, friend: Friend) {
        // TODO:
        // - This can be heavily optimized
        //   If friend not in friends, insert
        //   Else, only update if availability changed

        self.online.retain(|f| f.summonerId != friend.summonerId);
        self.busy.retain(|f| f.summonerId != friend.summonerId);
        self.away.retain(|f| f.summonerId != friend.summonerId);
        self.other.retain(|f| f.summonerId != friend.summonerId);
        self.offline.retain(|f| f.summonerId != friend.summonerId);
        self.insert(friend);
    }
}

impl From<Vec<Friend>> for Friends {
    fn from(friend_vec: Vec<Friend>) -> Self {
        let mut friends = Self::default();
        for friend in friend_vec.into_iter() {
            friends.insert(friend);
        }
        friends
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Data)]
pub struct Friend {
    availability: String,
    // displayGroupId: u16,
    // displayGroupName: String,
    gameName: String,
    // gameTag: String,
    // groupId: u16,
    // groupName: String,
    // icon: i16,
    // id: String,
    // isP2PConversationMuted: bool,
    // lastSeenOnlineTimestamp: Option<u64>,
    // lol: String, // unknown object?
    pub name: String,
    // note: String,
    // patchline: String,
    // pid: String,
    // platformId: String,
    // product: String,
    // productName: String,
    // puuid: String,
    // statusMessage: String,
    // summary: String,
    pub summonerId: u32,
    // time: u64
}

pub fn get_friends( http_connection: HttpConnection, event_sink: Arc<ExtEventSink>) {
    tokio::spawn(async move {
        let friends = Friends::from(
            get_request::<Vec<Friend>>(http_connection, "lol-chat/v1/friends").await.unwrap());
        event_sink.submit_command(
            SET_FRIENDS,
            SingleUse::new(friends),
            Target::Auto).unwrap();
    });
}