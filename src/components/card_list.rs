use makepad_widgets::*;
use serde::Deserialize;
use crate::components::space::CardDto;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::markdown::*;
    use crate::components::card_item::*;

    pub CardList = {{CardList}} {
        width: 260,
        height: Fill,
        margin: 10,
        padding: {top: 10, right: 10, bottom: 10, left: 10}
        draw_bg: {
            color: #F6E88BFF
        }
        flow: Down,
        spacing: 8,
        header_row = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8,
            align: {x: 0.0, y: 0.5}
            title = <H3> {
                text: "今天"
                draw_text: { color: #4A3E4EFF }
            }
            header_spacer = <View> { width: Fill, height: Fit }
            <Button> { text: "->" draw_text: { color: #6A5A72FF } }
            <Button> { text: "..." draw_text: { color: #6A5A72FF } }
        }
        <PortalList> {
            height: 256,
            flow: Down,

            CardItem = <CardItem> {}
        }
        divider = <View> {
            width: Fill,
            height: 1,
            draw_bg: { color: #E6D98AFF }
        }
        add_row = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8,
            <Button> { text: "+" draw_text: { color: #6A5A72FF } }
            <Button> { text: "添加卡片" draw_text: { color: #6A5A72FF } }
            add_spacer = <View> { width: Fill, height: Fit }
            <Button> { text: "=" draw_text: { color: #6A5A72FF } }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardList {
    #[deref]
    view: View,

    #[rust]
    cards_data: Vec<CardDto>,
}

impl CardList {
    pub fn set(&mut self, cards: &[CardDto], _cx: &mut Cx) {
        self.cards_data = cards.to_vec();
    }
}

impl Widget for CardList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let card_count = self.cards_data.len().max(1);
                list.set_item_range(cx, 0, card_count);
                while let Some(item_idx) = list.next_visible_item(cx) {
                    let item = list.item(cx, item_idx, live_id!(CardItem));
                    // Pass card data if available
                    if let Some(card) = self.cards_data.get(item_idx) {
                        // Update title directly
                        item.button(id!(title_text)).set_text(cx, &card.title);
                    }
                    item.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
}
