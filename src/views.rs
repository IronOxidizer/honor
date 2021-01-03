use app_dirs::AppDataType;
use druid::{Color, Data, KeyOrValue, Lens, LensExt, RenderContext, UnitPoint, Widget, WidgetExt,
    im::{Vector}, theme, widget::{Flex, Label, List, ViewSwitcher, Tabs, Axis, Button, Container, Scroll, TextBox,
        Painter, CrossAxisAlignment, ListIter}};

use super::util::*;
use super::lcu_api::*;

#[derive(Clone, PartialEq, Data)]
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

pub fn app_view_switcher() -> impl Widget<AppState> {
    Flex::column().with_child(
        ViewSwitcher::new(
            |data: &AppState, _env| data.view.to_owned(),
            |view, _data, _env| match view {
                //AppView::Connecting => unimplemented!(),
                AppView::Main => Box::new(view_main()),
                //AppView::ChampSelect => unimplemented!()
            }
        )).align_vertical(UnitPoint::CENTER)
}

// Consider using Buttons or lone Radio buttons and handle the logic manually
// RadioGroups require a static size, might be able to use lazy_static to do this
pub fn view_main() -> impl Widget<AppState> {
    fn summoner_card(summoner_name: &'static str, primary_role: &'static str, secondary_role: &'static str) -> impl Widget<AppState> {
        Container::new(
            Flex::column()
                .with_child(Label::new(summoner_name))
                .with_child(Label::new(primary_role))
                .with_child(Label::new(secondary_role))
                .padding(4.0)
        ).background(Color::grey8(128))
            .rounded(8.0)
            .padding((4.0, 8.0))
    }

    fn queue_list() -> impl Widget<Vector<lol_game_queues::Queue>> {
        List::new(|| {
            Label::new(|queue: &lol_game_queues::Queue, _env: &_| {
                queue.description.clone()
            }).background(Painter::new(|ctx, _, env| {
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

    fn friend_status_group(color: impl Into<KeyOrValue<Color>> + Clone + 'static) -> impl Widget<Vector<lol_chat::Friend>> {
        List::new(move || {
            Label::new(|friend: &lol_chat::Friend, _env: &_| friend.name.clone())
                .with_text_color(color.clone())
                .background(Painter::new(|ctx, _, env| {
                    let bounds = ctx.size().to_rect();
                    if ctx.is_active() {
                        ctx.fill(bounds, &Color::rgb8(32, 32, 32));
                    } else if ctx.is_hot() {
                        ctx.fill(bounds, &Color::rgb8(64, 64, 64))
                    }
                }))
                .on_click(move |_ctx, data, _env| eprintln!("{:?}", data))
                .expand_width()
        })
    }

    let queue_type_tabs = Tabs::new()
        .with_axis(Axis::Vertical)
        .with_tab("Ranked", queue_list().lens(lol_game_queues::Queues::ranked.in_arc()))
        .with_tab("Casual", queue_list().lens(lol_game_queues::Queues::casual.in_arc()))
        .with_tab("Versus AI", queue_list().lens(lol_game_queues::Queues::versus_ai.in_arc()))
        .lens(AppState::queues);

    let notification_scroll = Scroll::new(Label::new("notifications")).expand();
    let start_cancel = Button::new("Start/Cancel")
        .expand_width()
        .padding(2.0);
    let notif_start_cancel_col = Flex::column()
        .with_flex_child(notification_scroll, 1.0)
        .with_child(start_cancel);


    let chat_history = Scroll::new(Label::new("chat history")).expand();
    let chat_input = TextBox::new()
        .with_placeholder("Chat")
        .expand_width()
        .padding(2.0)
        .lens(AppState::chat_contents);
    let chat_col = Flex::column()
        .with_flex_child(chat_history, 1.0)
        .with_child(chat_input);

    // Seperate into 3 rows, League (different background or text color depending on status),
    //      Green = Online, Blue = In-Game/Queue, Red = Away 
    //      Other Game / Mobile, Offline
    let friends_list = Scroll::new(
        Flex::column()
            .with_child(Label::new("Online")
                .center().expand_width())
            .with_child(friend_status_group(Color::rgb8(32, 255, 32))
                .lens(lol_chat::Friends::online.in_arc()))
            .with_child(friend_status_group(Color::rgb8(92, 92, 255))
                .lens(lol_chat::Friends::busy.in_arc()))
            .with_child(friend_status_group(Color::rgb8(255, 32, 32))
                .lens(lol_chat::Friends::away.in_arc()))

            .with_child(Label::new("Other")
                .center().expand_width())
            .with_child(friend_status_group(Color::rgb8(192, 192, 160))
                .lens(lol_chat::Friends::other.in_arc())
            )

            .with_child(Label::new("Offline")
                .center().expand_width())
            .with_child(friend_status_group(Color::grey8(128))
                .lens(lol_chat::Friends::offline.in_arc()))
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .lens(AppState::friends)
    ).vertical()
        .expand();
    

    let top_row = Flex::row()
        .with_flex_child(queue_type_tabs, 2.0)
        .with_flex_child(notif_start_cancel_col, 1.0)
        .with_flex_child(chat_col, 1.0)
        .with_flex_child(friends_list, 1.0);//.debug_paint_layout();

    let summoner_cards = Scroll::new(
        Flex::row()
            .with_child(summoner_card("Maples", "Top", "Jungle"))
            .with_child(summoner_card("UnhingingPluto3", "Jungle", "Middle"))
            .with_child(summoner_card("Mackaron", "Support", "Middle"))
            .with_child(summoner_card("Chad", "Bottom", "Middle"))
            .with_child(summoner_card("XwMANBEARPIGwX", "Middle", "Bottom"))
    );//.debug_paint_layout().boxed();

    Flex::column()
        .with_flex_child(top_row, 1.0)
        .with_child(summoner_cards)
}