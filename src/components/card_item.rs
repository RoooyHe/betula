use crate::components::space::get_spaces_data;
use crate::components::card_list::get_current_space_index;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::card_tag::*;

    pub CardItem = {{CardItem}} {
        width: Fill,
        height: Fit,
        padding: {
            top: 8,
            right: 8,
            bottom: 8,
            left: 8
        }

        draw_bg: {
            color: #FFFFFFFF
        }

        flow: Down,
        spacing: 4,

        title_text = <Button> {
            text: "完成代码编辑"
            draw_text: {
                color: #3E3342FF
            }
        }
        
        // 固定标签用于测试
        <CardTag> {}
        
        meta_row = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8,
            date_text = <Label> { text: "2月7日" draw_text: { color: #6A5A72FF } }
            progress_text = <Label> { text: "0/2" draw_text: { color: #6A5A72FF } }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardItem {
    #[deref]
    view: View,
}

impl Widget for CardItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let space_index = get_current_space_index();
        let card_index = crate::components::card_list::get_current_card_index();
        
        let spaces_data = get_spaces_data();
        let spaces = spaces_data.lock().unwrap();
        
        let card = spaces.get(space_index)
            .and_then(|s| s.cards.get(card_index))
            .cloned();
        
        // 更新标题
        if let Some(ref card) = card {
            self.view.button(id!(title_text)).set_text(cx, &card.title);
        } else {
            self.view.button(id!(title_text)).set_text(cx, &format!("卡片 {}.{}", space_index, card_index));
        }
        
        self.view.draw_walk(cx, scope, walk)
    }
}
