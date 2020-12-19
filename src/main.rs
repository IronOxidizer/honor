#[macro_use]
extern crate lazy_static;

use druid::{AppLauncher, Widget, WidgetExt, WindowDesc, UnitPoint, Data, Lens,
    Env, EventCtx, Event, widget::{Flex, Label, Spinner, Controller}};
use std::sync::Arc;
use druid::ExtEventSink;

mod lcu_api;
mod util;
mod event_handlers;

use util::*;
use event_handlers::*;

pub const HOST: &str = "127.0.0.1";

struct EventHandler;
impl<W: Widget<AppState>> Controller<AppState, W> for EventHandler {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::WindowConnected => {tokio::spawn(event_handlers::update_summoner_name(
                data.connection_data.clone(), data.event_sink.clone())); ()},
            Event::Command(cmd) if cmd.is(SET_SUMMONER_NAME) =>
                if let Some(name) = cmd.get_unchecked(SET_SUMMONER_NAME).take()
                    {data.current_summoner_name = name},
            _ => ()
        };
        child.event(ctx, event, data, env)
    }
}
#[derive(Clone, Data, Lens)]
struct AppState {
    connection_data: ConnectionData,
    event_sink: Arc<ExtEventSink>,
    current_summoner_name: String
}

#[tokio::main]
async fn main() {
    let root_window = WindowDesc::new(build_root_widget)
        .title("Honor")
        // Keep title bar until window is controls and drag is implemented
        //.show_titlebar(false)
        .with_min_size((640., 360.));

    let launcher = AppLauncher::with_window(root_window);

    let app_state = AppState {
        connection_data: get_connection_data().expect("Could not get connection data"),
        event_sink: Arc::new(launcher.get_external_handle()),
        current_summoner_name: "Summoner Loading...".to_string(),
    };

    launcher
        .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}

fn build_root_widget() -> impl Widget<AppState> {
    let label = Label::raw()
        .lens(AppState::current_summoner_name)
        .padding(4.)
        .controller(EventHandler);

    let loading_spinner = Flex::column()
        .with_child(label)
        .with_child(Spinner::new());

    Flex::column()
        .with_child(loading_spinner)
        .align_vertical(UnitPoint::CENTER)
}