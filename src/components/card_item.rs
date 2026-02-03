use crate::components::space::CardDto;
use makepad_widgets::*;
use serde::Deserialize;

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

        <PortalList> {
            flow: Right,

            CardTag = <CardTag> {}
        }

        title_text = <Button> {
            text: "完成代码编辑"
            draw_text: {
                color: #3E3342FF
            }
        }
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

    #[rust]
    card_data: Option<CardDto>,
}

impl Widget for CardItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            // Set title text from card data
            if let Some(ref card) = self.card_data {
                self.view.button(id!(title_text)).set_text(cx, &card.title);
            }

            // 建议替换的片段（示例）
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(ref card) = self.card_data {
                    // 使用真实长度（不再强制至少为 1）
                    let tag_count = card.tags.len();
                    list.set_item_range(cx, 0, tag_count);

                    while let Some(item_idx) = list.next_visible_item(cx) {
                        let item = list.item(cx, item_idx, live_id!(CardTag));

                        // 额外的边界保护：保证 item_idx 在 tags 范围内
                        if item_idx < card.tags.len() {
                            if let Some(tag) = card.tags.get(item_idx) {
                                item.label(id!(tag_text)).set_text(cx, &tag.title);
                            } else {
                                // 防御性：若仍为 None，则置空显示
                                item.label(id!(tag_text)).set_text(cx, "");
                            }
                        } else {
                            // 超界：清空或隐藏该 item，避免渲染脏数据
                            item.label(id!(tag_text)).set_text(cx, "");
                        }

                        item.draw_all(cx, scope);
                    }
                } else {
                    // 没有 card 时确保列表为空
                    list.set_item_range(cx, 0, 0);
                }
            }
        }
        DrawStep::done()
    }
}
