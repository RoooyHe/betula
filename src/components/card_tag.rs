use makepad_widgets::*;
use serde::Deserialize;

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

#[derive(Clone, Deserialize)]
pub struct CardTagDto {
    pub id: i64,
    pub title: String,
    pub color: Option<String>,
}

#[derive(Live, LiveHook, Widget)]
pub struct CardTag {
    #[deref]
    view: View,

    #[rust]
    tag_data: Option<CardTagDto>,
}

impl CardTag {
    pub fn set(&mut self, tag: &CardTagDto, _cx: &mut Cx) {
        self.tag_data = Some(tag.clone());
    }
}

impl Widget for CardTag {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(ref tag) = self.tag_data {
            self.view
                .label(id!(tag_text))
                .set_text(cx, &tag.title);
        }
        self.view.draw_walk(cx, scope, walk)
    }
}
