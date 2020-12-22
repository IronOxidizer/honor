use druid::{Widget, WidgetExt, LensExt, Data, UnitPoint, Color,
    widget::{Flex, Label, List, ViewSwitcher, Tabs, Axis, Button, Container, Scroll}};

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

    let ranked_list = List::new(|| {
        Flex::row().with_child(
            Label::new(|queue: &lol_game_queues::Queue, _env: &_| {
               queue.description.clone()
            })
        )
    }).lens(lol_game_queues::Queues::ranked);

    let casual_list = List::new(|| {
        Flex::row().with_child(
            Label::new(|queue: &lol_game_queues::Queue, _env: &_| {
               queue.description.clone()
            })
        )
    }).lens(lol_game_queues::Queues::casual);

    let versus_ai_list = List::new(|| {
        Flex::row().with_child(
            Label::new(|queue: &lol_game_queues::Queue, _env: &_| {
               queue.description.clone()
            })
        )
    }).lens(lol_game_queues::Queues::versus_ai);

    let queue_type_tabs = Tabs::new()
        .with_axis(Axis::Vertical)
        .with_tab("Ranked", ranked_list)
        .with_tab("Casual", casual_list)
        .with_tab("Versus AI", versus_ai_list)
        .lens(AppState::queues);

    let notification_scroll = Scroll::new(Label::new("notifications")).expand();
    let start_cancel = Button::new("Start/Cancel");

    let notif_start_cancel_col = Flex::column()
        .with_flex_child(notification_scroll, 1.0)
        .with_child(start_cancel);

    let chat = Scroll::new(Label::new("chat")).expand();

    let friends_list = Scroll::new(Label::new("friends list")).expand();

    let top_row = Flex::row()
        .with_flex_child(queue_type_tabs, 1.6)
        .with_flex_child(notif_start_cancel_col, 1.0)
        .with_flex_child(chat, 1.0)
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