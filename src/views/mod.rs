use druid::{Data, Widget, WidgetExt, Env, EventCtx, Event, SingleUse, 
    widget::{ViewSwitcher, Controller}};

use super::lcu_api::*;
use super::AppState;

mod view_lobby;

use view_lobby::*;

#[derive(Copy, Clone, PartialEq, Data)]
pub enum AppView {
    //Connecting,
    Lobby,
    //ChampSelect
}

impl Default for AppView {
    fn default() -> Self {
        Self::Lobby
    }
}

pub fn build_root_widget() -> impl Widget<AppState> {
    // Top bar + borderless not feasible until we can emulate drag to reposition window
    ViewSwitcher::new(
        |data: &AppState, _env| data.view,
        |view, _data, _env| match view {
            AppView::Lobby => Box::new(view_lobby())
        }
    ).controller(EventHandler)
}

struct EventHandler;
impl<W: Widget<AppState>> Controller<AppState, W> for EventHandler {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::WindowConnected => {
                // Hydrate initial fields
                summoner::get_current_summoner(data.http_connection.clone(), data.event_sink.clone());
                game_queues::get_queues(data.http_connection.clone(), data.event_sink.clone());
                chat::get_friends(data.http_connection.clone(), data.event_sink.clone());
                lobby::get_lobby(data.http_connection.clone(), data.event_sink.clone());

                // Subscribe to events
                wamp_send(data.wamp_sink.clone(), MessageTypes::Subscribe, "OnJsonApiEvent_lol-chat_v1_friends");
                wamp_send(data.wamp_sink.clone(), MessageTypes::Subscribe, "OnJsonApiEvent_lol-lobby_v2_lobby");
            },
            Event::Command(cmd) => {
                if let Some(id) = cmd.get(POST_INVITE).and_then(SingleUse::take) {
                    lobby::post_lobby_invitations(data.http_connection.clone(), lobby::Invite::new(id));
                } else if let Some(summoner) = cmd.get(SET_CURRENT_SUMMONER).and_then(SingleUse::take) {
                    data.current_summoner = summoner
                } else if let Some(queues) = cmd.get(SET_QUEUES).and_then(SingleUse::take) {
                    data.queues = queues
                } else if let Some(friends) = cmd.get(SET_FRIENDS).and_then(SingleUse::take) {
                    data.friends = friends
                } else if let Some(lobby) = cmd.get(SET_LOBBY).and_then(SingleUse::take) {
                    data.lobby = lobby
                } else if let Some(friend) = cmd.get(UPDATE_FRIEND).and_then(SingleUse::take) {
                    data.friends.update(friend)
                }
            },
            _ => ()
        };
        child.event(ctx, event, data, env)
    }
}

