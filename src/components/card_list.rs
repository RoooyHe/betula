use makepad_widgets::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::components::space::get_spaces_data;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::card_item::*;

    pub CardList = {{CardList}} {
        width: 260,
        height: Fit,
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
                text: "空间标题"
                draw_text: { color: #4A3E4EFF }
            }
            header_spacer = <View> { width: Fill, height: Fit }
            <Button> { text: "->" draw_text: { color: #6A5A72FF } }
            <Button> { text: "..." draw_text: { color: #6A5A72FF } }
        }
        
        // 先使用固定的 CardItem 来测试
        <CardItem> {}
        <CardItem> {}
        
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

// 全局原子变量，用于在 PortalList 遍历时传递当前索引
static CURRENT_SPACE_INDEX: AtomicUsize = AtomicUsize::new(0);
static CURRENT_CARD_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn set_current_space_index(index: usize) {
    CURRENT_SPACE_INDEX.store(index, Ordering::SeqCst);
}

pub fn get_current_space_index() -> usize {
    CURRENT_SPACE_INDEX.load(Ordering::SeqCst)
}

pub fn set_current_card_index(index: usize) {
    CURRENT_CARD_INDEX.store(index, Ordering::SeqCst);
}

pub fn get_current_card_index() -> usize {
    CURRENT_CARD_INDEX.load(Ordering::SeqCst)
}

#[derive(Live, LiveHook, Widget)]
pub struct CardList {
    #[deref]
    view: View,
}

impl Widget for CardList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let space_index = get_current_space_index();
        let spaces_data = get_spaces_data();
        let spaces = spaces_data.lock().unwrap();
        
        // 更新标题
        if let Some(space) = spaces.get(space_index) {
            self.view.label(id!(header_row.title)).set_text(cx, &space.title);
        } else {
            self.view.label(id!(header_row.title)).set_text(cx, &format!("空间 {}", space_index));
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}
