use makepad_widgets::*;
use crate::components::space::get_spaces_data;
use crate::components::card_list::{get_current_space_index, get_current_card_index};

live_design! {
use link::theme::*;
use link::widgets::*;

pub CardTag = <RoundedView> {
        width: Fit,
        height: Fit,
        padding: {top: 2, right: 6, bottom: 2, left: 6}
        draw_bg: {
            color: #9FE7B4FF
            border_radius: 6.0
        }
        tag_text = <Label> {
            text: "非紧急"
            draw_text: {
                color: #2E5A45FF
            }
        }
    }
}

// 全局原子变量，用于在 PortalList 遍历时传递当前标签索引
static CURRENT_TAG_INDEX: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn set_current_tag_index(index: usize) {
    CURRENT_TAG_INDEX.store(index, std::sync::atomic::Ordering::SeqCst);
}

pub fn get_current_tag_index() -> usize {
    CURRENT_TAG_INDEX.load(std::sync::atomic::Ordering::SeqCst)
}

#[derive(Live, LiveHook, Widget)]
pub struct CardTag {
    #[deref]
    view: View,
}

impl Widget for CardTag {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let space_index = get_current_space_index();
        let card_index = get_current_card_index();
        let tag_index = get_current_tag_index();
        
        let spaces_data = get_spaces_data();
        let spaces = spaces_data.lock().unwrap();
        
        // 获取当前标签数据
        if let Some(tag) = spaces.get(space_index)
            .and_then(|s| s.cards.get(card_index))
            .and_then(|c| c.tags.get(tag_index)) {
            
            self.view.label(id!(tag_text)).set_text(cx, &tag.title);
            
            // 根据标签颜色设置背景色（如果有的话）
            if let Some(ref _color) = tag.color {
                // 这里可以根据颜色字符串设置背景色
                // 暂时使用默认颜色
            }
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}
