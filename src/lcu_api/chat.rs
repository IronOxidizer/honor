use serde::Deserialize;
use std::sync::Arc;
use druid::{Data, Lens, ExtEventSink, Selector, SingleUse, Target,
    im::Vector}; // All im types are wrapped with Arc thus are cheap to clone

use super::*;


// Seperate into 3 rows, League (different background or text color depending on status),
//      Green = Online, Blue = In-Game/Queue, Red = Away 
//      Other Game / Mobile, Offline

// Add group for in queue, turquois


#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Friends {
    pub online: Vector<Arc<Friend>>,
    pub busy: Vector<Arc<Friend>>,
    pub away: Vector<Arc<Friend>>,
    pub other: Vector<Arc<Friend>>,
    pub offline: Vector<Arc<Friend>>,
}

impl Friends {
    fn insert(&mut self, _friend: _Friend) {
        match _friend.availability.as_str() {
            "chat" => self.online.push_front(Arc::new(Friend::from(_friend))),
            "dnd" => self.busy.push_front(Arc::new(Friend::from(_friend))),
            "away" => self.away.push_front(Arc::new(Friend::from(_friend))),
            "offline" => self.offline.push_front(Arc::new(Friend::from(_friend))),
            _ => self.other.push_front(Arc::new(Friend::from(_friend)))
        };
    }

    pub fn update(&mut self, _friend: _Friend) {
        // TODO:
        // Compare len before and after retain, if changed, don't do other retains
        // Only insert if something was changed
        self.online.retain(|f| f.id != _friend.summonerId);
        self.busy.retain(|f| f.id != _friend.summonerId);
        self.away.retain(|f| f.id != _friend.summonerId);
        self.other.retain(|f| f.id != _friend.summonerId);
        self.offline.retain(|f| f.id != _friend.summonerId);
        self.insert(_friend);
    }
}

impl From<Vec<_Friend>> for Friends {
    fn from(_friend_vec: Vec<_Friend>) -> Self {
        let mut friends = Self::default();
        for friend in _friend_vec.into_iter() {
            friends.insert(friend);
        }
        friends
    }
}

#[derive(Clone, Default, Debug, Data, Lens)]
pub struct Friend {
    pub id: u32,
    pub name: String,
    pub availability: String
}

impl From<_Friend> for Friend {
    fn from(_friend: _Friend) -> Self {
        Self {
            id: _friend.summonerId,
            name: if _friend.name.is_empty() {
                    _friend.gameName
                } else {
                    _friend.name
                },
            availability: _friend.availability
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct _Friend {
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

pub fn get_friends( http_connection: HttpConnection, event_sink: Arc<ExtEventSink>) {
    tokio::spawn(async move {
        let friends = Friends::from(
            get_request::<Vec<_Friend>>(http_connection, "lol-chat/v1/friends").await.unwrap()
        );
        event_sink.submit_command(
            SET_FRIENDS,
            SingleUse::new(friends),
            Target::Auto).unwrap();
    });
}