use druid::{Data, Lens, ExtEventSink};
use std::sync::Arc;

// Don't need arc around Vector
use druid::im::{vector, Vector};

use super::lol_summoner;

// Everything in an Rc up to the scope of which the data is accquired / changing
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub event_sink: Arc<ExtEventSink>,
    pub connection: HttpConnectionState,
    //pub view: AppView,
    pub current_summoner: Vector<SummonerState>,
    //pub queues: Arc<Vec<lol_game_queues::Queue>>
}

impl AppState {
    pub fn new(event_sink: Arc<ExtEventSink>, connection: HttpConnectionState) -> Self {
        Self {
            event_sink: event_sink,
            connection: connection,
            current_summoner: Default::default(),
            // view: Default::default(),
            // queues: Default::default()
        }
    }
}

// Might have to use arc mutex/rwlock to pass around to threads in async
#[derive(Clone, Data, Lens)]
pub struct HttpConnectionState {
    pub client: Arc<reqwest::Client>,
    pub port: u16,
    pub token: String
}

#[derive(Clone, Data, Lens)]
pub struct SummonerState {
    pub display_name: String
}