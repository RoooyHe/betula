use makepad_widgets::*;
use serde::Deserialize;
use std::sync::{Arc, Mutex, OnceLock};

// 模块级共享状态
static SPACES_DATA: OnceLock<Arc<Mutex<Vec<SpaceDto>>>> = OnceLock::new();

#[inline]
pub fn get_spaces_data() -> Arc<Mutex<Vec<SpaceDto>>> {
    SPACES_DATA.get_or_init(|| Arc::new(Mutex::new(Vec::new()))).clone()
}

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::card_list::*;

    pub Space = {{Space}} {
        width: Fill,
        height: Fill,
        flow: Right,
        spacing: 10,
        draw_bg: {
            color: #F8F6FFFF
        }

        // 先使用固定的 CardList 来测试
        <CardList> {}
        <CardList> {}
        <CardList> {}
        <CardList> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Space {
    #[deref]
    view: View,
}

impl Widget for Space {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 为每个固定的 CardList 设置对应的空间索引
        crate::components::card_list::set_current_space_index(0);
        // 这里我们需要手动为每个 CardList 设置不同的索引
        // 但由于是固定的 UI 结构，我们先设置第一个为 0
        
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Clone, Deserialize)]
pub struct TagDto {
    pub id: i64,
    pub title: String,
    pub color: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct CardDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub tags: Vec<TagDto>,
}

#[derive(Clone, Deserialize)]
pub struct SpaceDto {
    pub id: i64,
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub canceled: Option<bool>,
    pub sort: Option<i32>,
    pub color: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    pub cards: Vec<CardDto>,
}
