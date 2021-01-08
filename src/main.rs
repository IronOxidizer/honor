#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
// ^ Only enabled on Windows + Release as it interferes with stderr making debugging harder

use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use tokio::sync::Mutex;
use druid::{AppLauncher, WindowDesc, Data, Lens, ExtEventSink};

mod lcu_api;
mod views;

use views::*;
use lcu_api::*;

pub const HOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> Result<()> {
    //start_lcu()?;
    // After attempting to start LCU, display spinner while waiting for connection, once connected kill LCU UX

    let root_window = WindowDesc::new(build_root_widget)
        .title("Honor")
        //.show_titlebar(false) // Keep title bar until window is controls and drag is implemented
        .with_min_size((640., 360.))
        .window_size((640., 360.));

    let launcher = AppLauncher::with_window(root_window);

    // Setup and initialize HTTP and WebSocket connection to LCU
    let (port, token) = get_lcu_connect_info()?;
    let (wamp_sink, wamp_stream) = lcu_api::connect_lcu_wamp(port, token.clone()).await?.split();
    tokio::spawn(lcu_api::wamp_poll_spin(wamp_stream, launcher.get_external_handle()));
    let http_connection = get_connection(port, token)?;
    
    // Initialize app state
    let app_state = AppState::new(
        Arc::new(Mutex::new(wamp_sink)),
        http_connection,
        Arc::new(launcher.get_external_handle()));

    // Launch app with app state
    launcher
        // .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");

    Ok(())
}

// Everything in an Rc up to the scope of which the data is accquired / changing
// When using Vector, Arc is not needed
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub wamp_sink: WampSink,
    pub http_connection: HttpConnection,
    pub event_sink: Arc<ExtEventSink>,
    pub view: AppView, // Implements copy, faster to not use Arc
    pub current_summoner: Arc<summoner::Summoner>,
    pub queues: Arc<game_queues::Queues>,
    pub friends: chat::Friends, // Don't wrap in Arc because each individual friend state might change because of websocket events
    pub chat_contents: String
}

impl AppState {
    pub fn new(wamp_sink: WampSink, http_connection: HttpConnection, event_sink: Arc<ExtEventSink>) -> Self {
        Self {
            wamp_sink,
            http_connection,
            event_sink,
            view: Default::default(),
            current_summoner: Default::default(),
            queues: Default::default(),
            friends: Default::default(),
            chat_contents: Default::default()
        }
    }
}
