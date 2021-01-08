use std::sync::Arc;
use druid::{Color, KeyOrValue, LensExt, RenderContext, Widget,
    WidgetExt, Env, SingleUse, Target,
    im::{Vector}, // All im types are wrapped with Arc thus are cheap to clone
    widget::{Flex, Label, List, Button, Container,
        Scroll, TextBox, Painter, CrossAxisAlignment}};

use super::*;
use super::super::*;

pub fn view_lobby() -> impl Widget<AppState> {
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
                    ctx.fill(bounds, &Color::grey8(32));
                } else if ctx.is_hot() {
                    ctx.fill(bounds, &Color::grey8(64))
                }
            })).on_click(|_ctx, data, _env| eprintln!("{:?}", data))
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
                        ctx.fill(bounds, &Color::grey8(32));
                    } else if ctx.is_hot() {
                        ctx.fill(bounds, &Color::grey8(64))
                    }
                }))
                .on_click(|ctx, data, _env|
                    ctx.get_external_handle().submit_command(
                        POST_INVITE,
                        SingleUse::new(data.summonerId),
                        Target::Auto).unwrap())
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

    // Use druid::either, when not in lobby show "not in lobby"
    // Consider exclusively using subscribe event for lobby
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