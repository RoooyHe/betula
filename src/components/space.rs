use makepad_widgets::*;
use serde::Deserialize;
use std::sync::mpsc::Receiver;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::card_list::*;

    pub Space = {{Space}} {
        width: Fill,
        height: Fill,
        flow: Right,
        spacing: 10,

        <PortalList> {
            height: 256,
            flow: Right,

            CardList = <CardList> {}
        }
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct Space {
    #[deref]
    view: View,

    #[rust]
    space_signal: SignalToUI,

    #[rust]
    space_rx: Option<Receiver<Vec<SpaceDto>>>,

    #[rust]
    spaces_data: Vec<SpaceDto>,
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

impl Widget for Space {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let space_count = self.spaces_data.len().max(1);
                list.set_item_range(cx, 0, space_count);
                while let Some(row_idx) = list.next_visible_item(cx) {
                    let row = list.item(cx, row_idx, live_id!(CardList));
                    // Update title from data
                    if let Some(space) = self.spaces_data.get(row_idx) {
                        row.view(id!(header_row.title)).set_text(cx, &space.title);
                    }
                    row.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
}

impl AppMain for Space {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            self.start_space_fetch();
        }
        if let Event::Signal = event {
            self.handle_space_signal(cx);
        }
    }
}

impl Space {
    fn start_space_fetch(&mut self) {
        if self.space_rx.is_some() {
            return;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.space_signal.clone();
        self.space_rx = Some(rx);
        std::thread::spawn(move || {
            let request = reqwest::blocking::get("http://localhost:8911/api/v1/space/byUserId/1");
            if let Ok(response) = request {
                if let Ok(spaces) = response.json::<Vec<SpaceDto>>() {
                    let _ = tx.send(spaces);
                    signal.set();
                }
            }
        });
    }

    fn handle_space_signal(&mut self, cx: &mut Cx) {
        if !self.space_signal.check_and_clear() {
            return;
        }
        let mut all_spaces = Vec::new();
        if let Some(rx) = &self.space_rx {
            while let Ok(spaces) = rx.try_recv() {
                all_spaces.push(spaces);
            }
        }
        for spaces in all_spaces {
            self.apply_spaces(cx, &spaces);
        }
    }

    fn apply_spaces(&mut self, cx: &mut Cx, spaces: &[SpaceDto]) {
        self.spaces_data = spaces.to_vec();

        // Update first CardList title directly
        if let Some(space) = spaces.first() {
            self.view
                .view(id!(CardList.header_row.title))
                .set_text(cx, &space.title);
        }
        if spaces.is_empty() {
            self.view
                .button(id!(space_tabs.space_item_1))
                .set_text(cx, "暂无空间");
        }
    }
}
