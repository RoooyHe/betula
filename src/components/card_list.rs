use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::card_item::CardItem;

    pub CardList = {{CardList}} {
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5,
            
            cards = <PortalList> {
                flow: Down,
                spacing: 5,

                Card = <CardItem> {}
            }
            
            // 新卡片输入框
            new_card_input = <View> {
                width: Fill,
                height: Fit,
                margin: {bottom: 8, top: 5},
                padding: 10,
                flow: Down,
                spacing: 5,
                visible: false,

                <RoundedView> {
                    width: Fill,
                    height: 40,
                    draw_bg: {
                        color: #F0F8FFFF
                    }
                    
                    new_card_text_input = <TextInput> {
                        width: Fill,
                        height: Fill,
                        text: "",
                        draw_text: {
                            color: #333333FF,
                            text_style: {
                                font_size: 14.0,
                            }
                        }
                        draw_bg: {
                            color: #FFFFFFFF
                        }
                    }
                }
                
                <Label> {
                    text: "输入卡片标题，按回车保存",
                    draw_text: {
                        color: #888888FF,
                        text_style: {
                            font_size: 11.0,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CardList {
    #[deref]
    view: View,
}

impl Widget for CardList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理新卡片输入框的事件
        if let Event::Actions(actions) = event {
            // 处理回车键创建新卡片
            if let Some((text, _)) = self.view.text_input(id!(new_card_text_input)).returned(actions) {
                if !text.trim().is_empty() {
                    let state = scope.data.get_mut::<crate::app::State>().unwrap();
                    let space_idx = *scope.props.get::<usize>().unwrap();
                    let space_id = state.spaces_data[space_idx].id;
                    
                    // 设置待创建的卡片
                    state.pending_create_card = Some((space_id, text.trim().to_string()));
                    println!("设置待创建卡片: '{}' 到空间: {}", text.trim(), space_id);
                }
            }
            
            // 处理输入框文本变化事件
            if let Some(text) = self.view.text_input(id!(new_card_text_input)).changed(actions) {
                let state = scope.data.get_mut::<crate::app::State>().unwrap();
                let space_idx = *scope.props.get::<usize>().unwrap();
                let space_id = state.spaces_data[space_idx].id;
                
                // 更新输入框状态
                state.new_card_inputs.insert(space_id, text);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // 先获取需要的数据
        let (_space_idx, _space_id, has_new_input, current_text, cards_data) = {
            let state = scope.data.get_mut::<crate::app::State>().unwrap();
            let space_idx = *scope.props.get::<usize>().unwrap();
            let space_id = state.spaces_data[space_idx].id;
            let has_new_input = state.new_card_inputs.contains_key(&space_id);
            let default_text = String::new();
            let current_text = state.new_card_inputs.get(&space_id).unwrap_or(&default_text).clone();
            let cards_data = state.spaces_data[space_idx].cards.clone();
            (space_idx, space_id, has_new_input, current_text, cards_data)
        };

        // 处理新卡片输入框的可见性
        if has_new_input {
            self.view.view(id!(new_card_input)).apply_over(cx, live! { visible: true });
            
            let text_input = self.view.text_input(id!(new_card_text_input));
            text_input.set_text(cx, &current_text);
        } else {
            self.view.view(id!(new_card_input)).apply_over(cx, live! { visible: false });
        }

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, cards_data.len());

                while let Some(card_idx) = list.next_visible_item(cx) {
                    if card_idx >= cards_data.len() {
                        continue;
                    }

                    let card_item = list.item(cx, card_idx, live_id!(Card));
                    let card = &cards_data[card_idx];

                    // 设置卡片标题输入框
                    card_item
                        .text_input(id!(card_title_input))
                        .set_text(cx, &card.title);

                    // 设置标签信息
                    let tags_text = if card.tags.is_empty() {
                        "无标签".to_string()
                    } else {
                        card.tags
                            .iter()
                            .map(|tag| tag.title.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    card_item
                        .label(id!(card_tags))
                        .set_text(cx, &format!("标签: {}", tags_text));

                    card_item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }
}
