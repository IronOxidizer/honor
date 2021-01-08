use std::sync::Arc;
use druid::{Color, Data, KeyOrValue, LensExt, RenderContext, Widget,
    WidgetExt, Env, EventCtx, Event, SingleUse,
    im::{Vector}, // All im types are wrapped with Arc thus are cheap to clone
    widget::{Flex, Label, List, ViewSwitcher, Button, Container,
        Scroll, TextBox, Painter, CrossAxisAlignment, Controller}};

use super::lcu_api::*;
use super::AppState;

#[derive(Copy, Clone, PartialEq, Data)]
pub enum AppView {
    //Connecting,
    Main,
    //ChampSelect
}

impl Default for AppView {
    fn default() -> Self {
        Self::Main
    }
}

pub fn build_root_widget() -> impl Widget<AppState> {
    // Top bar + borderless not feasible until we can emulate drag to reposition window
    ViewSwitcher::new(
        |data: &AppState, _env| data.view,
        |view, _data, _env| match view {
            AppView::Main => Box::new(view_main())
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

                // Subscribe to events
                wamp_send(data.wamp_sink.clone(), MessageTypes::Subscribe, "OnJsonApiEvent_lol-chat_v1_friends");
            },
            Event::Command(cmd) => {
                if let Some(summoner) = cmd.get(SET_CURRENT_SUMMONER).and_then(SingleUse::take) {
                    data.current_summoner = summoner
                } else if let Some(queues) = cmd.get(SET_QUEUES).and_then(SingleUse::take) {
                    data.queues = queues
                } else if let Some(friends) = cmd.get(SET_FRIENDS).and_then(SingleUse::take) {
                    data.friends = friends
                } else if let Some(_friend) = cmd.get(UPDATE_FRIEND).and_then(SingleUse::take) {
                    data.friends.update(_friend)
                }
            },
            _ => ()
        };
        child.event(ctx, event, data, env)
    }
}

pub fn view_main() -> impl Widget<AppState> {
    fn summoner_card(summoner_name: &'static str, primary_role: &'static str, secondary_role: &'static str) -> impl Widget<AppState> {
        Container::new(
            Flex::column()
                .with_child(Label::new(summoner_name))
                .with_child(Label::new(primary_role))
                .with_child(Label::new(secondary_role))
                .padding(4.0)
        ).background(Color::grey8(58))
            .rounded(8.0)
            .padding((4.0, 8.0))
    }

    fn queue_list() -> impl Widget<Vector<game_queues::Queue>> {
        List::new(|| {
            Label::new(|queue: &game_queues::Queue, _env: &Env | {
                queue.description.clone()
            }).background(Painter::new(|ctx, _data, _env| {
                let bounds = ctx.size().to_rect();
                if ctx.is_active() {
                    ctx.fill(bounds, &Color::rgb8(32, 32, 32));
                } else if ctx.is_hot() {
                    ctx.fill(bounds, &Color::rgb8(64, 64, 64))
                }
            })).on_click(move |_ctx, data, _env| eprintln!("{:?}", data))
            .expand_width()
        })
    }

    fn friend_status_group(color: impl Into<KeyOrValue<Color>> + Clone + 'static) -> impl Widget<Vector<Arc<chat::Friend>>> {
        List::new(move || {
            Label::new(|friend: &Arc<chat::Friend>, _env: &Env | friend.name.clone())
                .with_text_color(color.clone())
                .background(Painter::new(|ctx, _data, _env| {
                    let bounds = ctx.size().to_rect();
                    if ctx.is_active() {
                        ctx.fill(bounds, &Color::rgb8(32, 32, 32));
                    } else if ctx.is_hot() {
                        ctx.fill(bounds, &Color::rgb8(64, 64, 64))
                    }
                }))
                // Try to get http connection from environment
                .on_click(move |_ctx, data, _env|lobby::post_lobby_invitations(data, data.id))// lobby::post_lobby_invitations(_env.get_all(), data.id))
                .expand_width()
        })
    }

    let queue_lists = Scroll::new(
        Flex::column()
            .with_child(Label::new("Ranked")
                .center().expand_width())
            .with_child(queue_list().lens(game_queues::Queues::ranked.in_arc()))
            .with_child(Label::new("Casual")
                .center().expand_width())
            .with_child(queue_list().lens(game_queues::Queues::casual.in_arc()))
            .with_child(Label::new("Versus AI")
                .center().expand_width())
            .with_child(queue_list().lens(game_queues::Queues::versus_ai.in_arc()))
            .lens(AppState::queues)
    ).vertical()
        .expand();

    let notification_scroll = Scroll::new(Label::new("notifications")).expand();
    let chat_scroll = Scroll::new(Label::new("chat history")).expand();
    let start_cancel = Button::new("Start/Cancel")
        .expand_width()
        .padding(2.0);
    let chat_input = TextBox::new()
        .with_placeholder("Chat")
        .expand_width()
        .padding(2.0)
        .lens(AppState::chat_contents);

    let notif_chat_col = Flex::column()
        .with_flex_child(notification_scroll, 1.0)
        .with_flex_child(chat_scroll, 1.0)
        .with_child(Flex::row()
            .with_flex_child(start_cancel, 1.0)
            .with_flex_child(chat_input, 2.0)
        );

    // Seperate into 3 rows, League (different background or text color depending on status),
    //      Green = Online, Blue = In-Game/Queue, Red = Away 
    //      Other Game / Mobile, Offline
    let friend_lists = Scroll::new(
        Flex::column()
            .with_child(Label::new("Online")
                .center().expand_width())
            .with_child(friend_status_group(Color::rgb8(32, 255, 32))
                .lens(chat::Friends::online))
            .with_child(friend_status_group(Color::rgb8(92, 92, 255))
                .lens(chat::Friends::busy))
            .with_child(friend_status_group(Color::rgb8(255, 32, 32))
                .lens(chat::Friends::away))

            .with_child(Label::new("Other")
                .center().expand_width())
            .with_child(friend_status_group(Color::rgb8(192, 192, 160))
                .lens(chat::Friends::other)
            )

            .with_child(Label::new("Offline")
                .center().expand_width())
            .with_child(friend_status_group(Color::grey8(128))
                .lens(chat::Friends::offline))
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .lens(AppState::friends)
    ).vertical()
        .expand();
    

    let top_row = Flex::row()
        .with_flex_child(queue_lists, 1.0)
        .with_flex_child(notif_chat_col, 2.0)
        .with_flex_child(friend_lists, 1.0); //.debug_paint_layout();

    let summoner_cards = Scroll::new(
        Flex::row()
            .with_child(summoner_card("Player1", "Top", "Jungle"))
            .with_child(summoner_card("PlayerWithAReallyLongName2", "Jungle", "Middle"))
            .with_child(summoner_card("Player3", "Support", "Middle"))
            .with_child(summoner_card("Player4", "Bottom", "Middle"))
            .with_child(summoner_card("PlayerWithALongName5", "Middle", "Bottom"))
            .with_child(summoner_card("PlayerWithALongName6", "Middle", "Bottom"))
    ); //.debug_paint_layout().boxed();

    Flex::column()
        .with_flex_child(top_row, 1.0)
        .with_child(summoner_cards)
}