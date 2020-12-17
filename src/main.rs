#[macro_use]
extern crate lazy_static;

use futures::future;
use async_std::{task, net::TcpStream};
use anyhow::Result;
use async_native_tls::TlsConnector;
use async_tungstenite::client_async;
use async_tungstenite::tungstenite::handshake::client::Request;
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc, UnitPoint,
    widget::{Button, Flex, Label, Spinner}};
mod lcu_api;
mod util;
use util::*;


async fn run() {
    //run_lcu().expect("Error running LCU");
    //connect_to_lcu().await.expect("Couldn't connect to LCU");

    let main_window = WindowDesc::new(ui_builder)
        .title("Honor")
        .with_min_size((640., 360.));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<()> {
    let loading_spinner = Flex::column()
        .with_child(Label::new("Waiting for Riot LCU").padding(4.))
        .with_child(Spinner::new());

    Flex::column()
        .with_child(loading_spinner)
        .align_vertical(UnitPoint::CENTER)
}

fn main() {
    task::block_on(run());
}