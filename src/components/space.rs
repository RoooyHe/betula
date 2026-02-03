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

        space_list: <PortalList> {
            width: Fill,
            height: Fill,
            flow: Right,
            spacing: 10,

            CardList = <CardList> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Space {
    #[deref]
    view: View,
    
    #[live]
    space_list: PortalList,
}

impl Widget for Space {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
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
    pub end_time: Option<String>,
    pub tags: Vec<TagDto>,
}

#[derive(Clone, Deserialize)]
pub struct SpaceDto {
    pub id: i64,
    pub title: String,
    pub user_id: String,
    pub canceled: Option<bool>,
    pub sort: Option<i32>,
    pub color: Option<String>,
    pub sort_by: Option<String>,
    pub cards: Vec<CardDto>,
}
