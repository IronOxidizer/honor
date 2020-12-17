use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};
use futures::{future, pin_mut, StreamExt};

use async_std::io;
use async_std::prelude::*;
use async_std::task;
use async_tungstenite::async_std::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
use async_tungstenite::WebSocketStream;
use async_std::net::TcpStream;

fn kill_ux(wss: WebSocketStream<TcpStream>) {
    let (write, read) = wss.split();
}