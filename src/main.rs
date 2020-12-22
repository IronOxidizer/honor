#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc, Env, EventCtx, Event, widget::Controller};

mod lcu_api;
mod util;
mod event_handlers;
mod views;


use util::*;
use event_handlers::*;
use views::app_view_switcher;
use views::view_main;
pub const HOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() {
    let root_window = WindowDesc::new(build_root_widget)
        .title("Honor")
        // Keep title bar until window is controls and drag is implemented
        //.show_titlebar(false)
        .with_min_size((640., 360.));

    let launcher = AppLauncher::with_window(root_window);

    let app_state = AppState::new(
        Arc::new(launcher.get_external_handle()),
        get_connection_data().expect("Could not get connection data")
    );

    launcher
        // .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}

fn build_root_widget() -> impl Widget<AppState> {
    // Top bar, not feasible until we can emulate drag to reposition window
    // Flex::column()
    // .with_child(
    //     Flex::row()
    //         .with_child(Label::new("Top Bar"))
    //         .with_flex_spacer(1.0)
    //         .with_child(Label::new("o - x"))
    //         .main_axis_alignment(MainAxisAlignment::SpaceBetween) // Maybe remove?
    //         .cross_axis_alignment(CrossAxisAlignment::Center) // Maybe change to CrossAxis::Start
    //         .border(Color::WHITE, 0.5))
    // .with_child(Flex::row()
    //     .with_child(app_view_switcher())
    // ).controller(EventHandler)

    //app_view_switcher()
    view_main()
        .controller(EventHandler)
    
    // Tabs::new()
    //     .with_axis(Axis::Vertical)
    //     .with_tab("tab0", Label::new("This is tab0"))
    //     .with_tab("tab1", Label::new("This is tab1"))
    //     .with_tab("tab2", Label::new("This is tab2"))
}

struct EventHandler;
impl<W: Widget<AppState>> Controller<AppState, W> for EventHandler {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::WindowConnected => {
                tokio::spawn(event_handlers::update_current_summoner(
                    data.event_sink.clone(), data.connection.clone())); 
                    
                tokio::spawn(event_handlers::update_queues(
                    data.event_sink.clone(), data.connection.clone()));
                
                ()
            },
            Event::Command(cmd) => {
                if cmd.is(SET_CURRENT_SUMMONER) {
                    if let Some(summoner) = cmd.get_unchecked(SET_CURRENT_SUMMONER).take()
                        {data.current_summoner = Arc::new(summoner)}
                } else if cmd.is(SET_QUEUES) {
                    if let Some(queues) = cmd.get_unchecked(SET_QUEUES).take()
                        {data.queues = queues}
                }
            },
            _ => ()
        };
        child.event(ctx, event, data, env)
    }
}